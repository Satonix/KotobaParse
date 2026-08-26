use anyhow::Context;
use clap::{Parser, Subcommand};
use kotoba_core::{
    decode_source_bytes, diagnose_source, encode_text, extract_binary_text_entries, extract_bytes,
    language_spec, parse_source, preview_rebuild_bytes, rebuild_bytes_with_report,
    rebuild_bytes_with_report_options, summarize_source, KotobaCharacterSubstitution,
    KotobaPatchInput,
};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "kotoba", version, about = "KotobaParse DSL CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Parse and validate a .kotoba parser.
    Check {
        parser: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Validate a .kotoba parser and print editor-friendly diagnostics JSON.
    Diagnose { parser: PathBuf },
    /// Print the compiled parser spec as JSON.
    Spec { parser: PathBuf },
    /// Print the formal Kotoba language feature matrix as JSON.
    LanguageSpec,
    /// Print KotobaParse editor autocomplete items as JSON.
    Completions {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        context: Option<String>,
    },
    /// Print an editor-friendly parser summary as JSON.
    Summary { parser: PathBuf },
    /// Extract entries from a script using a .kotoba parser.
    Extract {
        parser: PathBuf,
        input: PathBuf,
        out: PathBuf,
    },
    /// Emit a single editor payload: diagnostics, summary, extraction report and sample entries.
    EditorProbe {
        parser: PathBuf,
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// Rebuild a script using KotobaPatchInput JSON.
    Rebuild {
        parser: PathBuf,
        input: PathBuf,
        patches: PathBuf,
        out: PathBuf,
        /// Replace characters unsupported by the Recipe output encoding with '?'.
        #[arg(long)]
        lossy: bool,
        /// Write the structured rebuild report to a JSON file.
        #[arg(long)]
        report: Option<PathBuf>,
        /// Apply per-project text or raw-byte substitutions to translated captures.
        #[arg(long)]
        substitutions: Option<PathBuf>,
    },
    /// Preview rebuild changes without writing a rebuilt script.
    Preview {
        parser: PathBuf,
        input: PathBuf,
        patches: PathBuf,
        out: PathBuf,
    },
    /// Extract and rebuild without patches. Useful for round-trip sanity checks.
    Roundtrip {
        parser: PathBuf,
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Extract likely translatable ASCII strings from a binary script.
    Strings {
        input: PathBuf,
        out: PathBuf,
        #[arg(long, default_value_t = 4)]
        min_len: usize,
    },
}

fn kotoba_completions_payload(context: Option<&str>) -> serde_json::Value {
    let requested_context = context
        .unwrap_or("global")
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_")
        .replace(' ', "_");
    let mut items = Vec::new();

    macro_rules! item {
        ($ctx:expr, $label:expr, $kind:expr, $detail:expr) => {{
            items.push(serde_json::json!({
                "context": $ctx,
                "label": $label,
                "kind": $kind,
                "insert": $label,
                "detail": $detail,
            }));
        }};
        ($ctx:expr, $label:expr, $kind:expr, $detail:expr, $snippet:expr) => {{
            items.push(serde_json::json!({
                "context": $ctx,
                "label": $label,
                "kind": $kind,
                "insert": $label,
                "snippet": $snippet,
                "detail": $detail,
            }));
        }};
    }

    item!(
        "root",
        "parser Nome:",
        "keyword",
        "Inicia uma Recipe canônica.",
        "parser Nome:\n    file \".txt\"\n    encoding utf8"
    );
    for context in ["root", "parser_body"] {
        item!(context, "file", "property", "Define a extensão alvo.");
        item!(
            context,
            "encoding",
            "property",
            "Define o encoding do arquivo reconstruído."
        );
        item!(
            context,
            "type Nome = matches",
            "property",
            "Cria um tipo validado por regex."
        );
        item!(
            context,
            "type Nome = like",
            "property",
            "Cria um tipo com padrão amigável."
        );
        item!(
            context,
            "read:",
            "block",
            "Define a unidade de leitura.",
            "read:\n    records as lines"
        );
        item!(
            context,
            "numbered lines:",
            "block",
            "Remove um identificador delimitado de cada linha.",
            "numbered lines:\n    id between \"<\" and \">\""
        );
        item!(
            context,
            "ignore:",
            "block",
            "Filtra records não traduzíveis.",
            "ignore:\n    empty\n    starts with \";\""
        );
        item!(
            context,
            "protect:",
            "block",
            "Protege tokens durante a tradução.",
            "protect:\n    between \"[\" and \"]\""
        );
        item!(context, "voice:", "block", "Captura e lembra voice-id.");
        item!(context, "speaker:", "block", "Captura e lembra speaker.");
        item!(context, "dialogue:", "block", "Extrai falas.");
        item!(
            context,
            "dialogue Nome:",
            "block",
            "Nomeia uma forma específica de diálogo."
        );
        item!(context, "text:", "block", "Extrai narração.");
        item!(
            context,
            "text Nome:",
            "block",
            "Nomeia uma forma específica de texto."
        );
        item!(context, "choice:", "block", "Extrai escolhas.");
        item!(
            context,
            "choice Nome:",
            "block",
            "Nomeia uma forma específica de escolha."
        );
        item!(
            context,
            "ignore Nome:",
            "block",
            "Ignora um padrão na ordem declarada."
        );
        item!(
            context,
            "json Nome:",
            "block",
            "Extrai objetos por caminho JSON."
        );
    }

    for label in [
        "utf8",
        "utf8-bom",
        "cp932",
        "shift-jis",
        "utf16le",
        "utf16be",
        "windows-1252",
        "ansi",
    ] {
        item!(
            "encoding",
            label,
            "value",
            "Encoding de saída aceito pelo runtime."
        );
    }
    for label in [
        "\".txt\"", "\".ks\"", "\".sc\"", "\".ast\"", "\".bs5\"", "\".nut\"",
    ] {
        item!("file", label, "value", "Extensão alvo.");
    }

    for label in [
        "empty",
        "starts with",
        "starts with any",
        "contains",
        "contains any",
        "ends with",
        "ends with any",
        "equals",
        "equals any",
        "matches",
        "like",
        "like any",
        "asset",
    ] {
        item!("ignore", label, "statement", "Condição canônica de filtro.");
    }
    for label in ["between", "literal", "matches"] {
        item!("protect", label, "statement", "Proteção canônica.");
    }
    for label in [
        "records as lines",
        "records as blocks",
        "records as segmented lines",
        "records as binary",
        "block starts with",
        "block ends with",
        "block ends when braces close",
        "segments separated by",
        "choice segments separated by",
        "fields",
        "source field",
        "patch field",
        "magic",
        "length",
        "encoding",
        "min length",
    ] {
        item!(
            "read",
            label,
            "statement",
            "Configuração canônica de leitura."
        );
    }
    for label in [
        "section",
        "when starts with",
        "capture voice after",
        "capture speaker from attribute",
        "capture voice from attribute",
        "like",
        "like any",
        "matches",
        "remember voice",
        "remember speaker",
        "skip",
    ] {
        item!("voice", label, "statement", "Comando canônico de voice.");
    }
    for label in [
        "section",
        "when starts with",
        "when previous is voice",
        "when next is quoted",
        "capture speaker between",
        "capture speaker from attribute",
        "capture voice from attribute",
        "remember speaker",
        "skip",
    ] {
        item!(
            "speaker",
            label,
            "statement",
            "Comando canônico de speaker."
        );
    }
    for label in [
        "section",
        "field",
        "when starts with",
        "capture id as number",
        "capture voice as optional",
        "capture voice if like",
        "capture speaker between",
        "capture speaker before quoted text",
        "capture speaker before rest text if starts with any",
        "capture text after",
        "capture text between",
        "capture text as",
        "speaker fallback",
        "save as",
        "save otherwise as",
        "patch",
        "patch speaker",
    ] {
        item!(
            "dialogue",
            label,
            "statement",
            "Comando canônico de diálogo."
        );
    }
    for label in [
        "field",
        "section",
        "capture text between",
        "capture text as",
        "save as",
        "patch",
    ] {
        item!("text", label, "statement", "Comando canônico de texto.");
    }
    for label in [
        "field",
        "when starts with",
        "capture text as",
        "capture choices as quoted",
        "save as",
        "patch",
    ] {
        item!("choice", label, "statement", "Comando canônico de escolha.");
    }
    for context in ["dialogue", "text", "choice", "ignore"] {
        for label in [
            "when matches",
            "when format",
            "when starts with",
            "when previous is voice",
            "when previous is speaker",
            "when exists",
            "when not exists",
            "when next is quoted",
            "when text is quoted",
            "capture",
            "save as",
            "patch",
            "context is",
            "remember",
            "forget",
            "skip",
        ] {
            item!(
                context,
                label,
                "statement",
                "Comando combinável de bloco semântico."
            );
        }
    }
    for label in ["patch speaker", "speaker fallback", "speaker is", "text is"] {
        item!(
            "dialogue",
            label,
            "statement",
            "Comando combinável de diálogo."
        );
    }
    for label in ["entries", "text", "speaker", "context", "id"] {
        item!(
            "json",
            label,
            "statement",
            "Mapeamento canônico de campo JSON."
        );
    }

    let filtered = if requested_context == "global" || requested_context.is_empty() {
        items
    } else {
        items
            .into_iter()
            .filter(|item| {
                item.get("context").and_then(|value| value.as_str())
                    == Some(requested_context.as_str())
            })
            .collect()
    };

    serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "context": requested_context,
        "items": filtered,
    })
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Check { parser, json } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            if json {
                let report = diagnose_source(&parser_src);
                println!("{}", serde_json::to_string_pretty(&report)?);
                if !report.ok {
                    anyhow::bail!("parser contains errors");
                }
            } else {
                let report = diagnose_source(&parser_src);
                if !report.ok {
                    for diagnostic in &report.diagnostics {
                        eprintln!(
                            "{}: {}",
                            diagnostic.severity.to_uppercase(),
                            diagnostic.message
                        );
                    }
                    anyhow::bail!("parser contains errors");
                }
                let warning_count = report
                    .diagnostics
                    .iter()
                    .filter(|d| d.severity == "warning")
                    .count();
                let summary = report
                    .summary
                    .as_ref()
                    .context("diagnostic report missing summary")?;
                eprintln!(
                    "OK: {} ({}) rules={} warnings={}",
                    summary.name,
                    summary.id,
                    summary.rules.len(),
                    warning_count
                );
            }
        }
        Command::Diagnose { parser } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let report = diagnose_source(&parser_src);
            println!("{}", serde_json::to_string_pretty(&report)?);
            if !report.ok {
                anyhow::bail!("parser contains errors");
            }
        }
        Command::Spec { parser } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let spec = parse_source(&parser_src)?;
            println!("{}", serde_json::to_string_pretty(&spec)?);
        }
        Command::LanguageSpec => {
            println!("{}", serde_json::to_string_pretty(&language_spec())?);
        }
        Command::Completions { json, context } => {
            let payload = kotoba_completions_payload(context.as_deref());
            if json {
                println!("{}", serde_json::to_string_pretty(&payload)?);
            } else {
                if let Some(items) = payload.get("items").and_then(|value| value.as_array()) {
                    for item in items {
                        if let Some(label) = item.get("label").and_then(|value| value.as_str()) {
                            println!("{}", label);
                        }
                    }
                }
            }
        }
        Command::Summary { parser } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let summary = summarize_source(&parser_src)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Command::Extract { parser, input, out } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let spec = parse_source(&parser_src)?;
            let input_bytes =
                fs::read(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let (entries, report) = extract_bytes(&input_bytes, &spec)?;
            fs::write(&out, serde_json::to_vec_pretty(&entries)?)?;
            eprintln!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::EditorProbe {
            parser,
            input,
            out,
            limit,
        } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let diagnostics = diagnose_source(&parser_src);
            let spec = parse_source(&parser_src)?;
            let input_bytes =
                fs::read(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let (entries, extract_report) = extract_bytes(&input_bytes, &spec)?;
            let sample_entries = entries.into_iter().take(limit).collect::<Vec<_>>();
            let payload = serde_json::json!({
                "diagnostics": diagnostics,
                "extract_report": extract_report,
                "sample_entries": sample_entries,
            });
            let bytes = serde_json::to_vec_pretty(&payload)?;
            if let Some(out) = out {
                fs::write(out, bytes)?;
            } else {
                println!("{}", String::from_utf8_lossy(&bytes));
            }
        }
        Command::Rebuild {
            parser,
            input,
            patches,
            out,
            lossy,
            report,
            substitutions,
        } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let spec = parse_source(&parser_src)?;
            let input_bytes =
                fs::read(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let patches: Vec<KotobaPatchInput> = serde_json::from_slice(
                &fs::read(&patches)
                    .with_context(|| format!("failed to read {}", patches.display()))?,
            )?;
            let substitutions: Vec<KotobaCharacterSubstitution> = match substitutions {
                Some(path) => serde_json::from_slice(
                    &fs::read(&path)
                        .with_context(|| format!("failed to read {}", path.display()))?,
                )
                .with_context(|| format!("invalid substitutions file {}", path.display()))?,
                None => Vec::new(),
            };
            let (rebuilt, rebuild_report) = rebuild_bytes_with_report_options(
                &input_bytes,
                &spec,
                &patches,
                lossy,
                &substitutions,
            )?;
            fs::write(out, rebuilt)?;
            if let Some(report_path) = report {
                fs::write(report_path, serde_json::to_vec_pretty(&rebuild_report)?)?;
            }
            eprintln!("{}", serde_json::to_string_pretty(&rebuild_report)?);
        }
        Command::Preview {
            parser,
            input,
            patches,
            out,
        } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let spec = parse_source(&parser_src)?;
            let input_bytes =
                fs::read(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let patches: Vec<KotobaPatchInput> = serde_json::from_slice(
                &fs::read(&patches)
                    .with_context(|| format!("failed to read {}", patches.display()))?,
            )?;
            let preview = preview_rebuild_bytes(&input_bytes, &spec, &patches)?;
            fs::write(out, serde_json::to_vec_pretty(&preview)?)?;
            eprintln!("{}", serde_json::to_string_pretty(&preview.report)?);
        }
        Command::Roundtrip { parser, input, out } => {
            let parser_src = fs::read_to_string(&parser)
                .with_context(|| format!("failed to read {}", parser.display()))?;
            let spec = parse_source(&parser_src)?;
            let input_bytes =
                fs::read(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let (rebuilt, report) = rebuild_bytes_with_report(&input_bytes, &spec, &[])?;
            if let Some(out) = out {
                fs::write(out, rebuilt)?;
                eprintln!("{}", serde_json::to_string_pretty(&report)?);
            } else if rebuilt == input_bytes {
                eprintln!("OK: round-trip sem alterações");
            } else {
                let original_text = decode_source_bytes(&input_bytes)?;
                let rebuilt_text = decode_source_bytes(&rebuilt).or_else(|_| {
                    String::from_utf8(rebuilt.clone()).map_err(|e| {
                        kotoba_core::KotobaError::Encoding {
                            encoding: "utf-8".into(),
                            message: e.to_string(),
                        }
                    })
                })?;
                if original_text == rebuilt_text {
                    let normalized = encode_text(&rebuilt_text, spec.encoding.as_deref())?;
                    if normalized == input_bytes {
                        eprintln!("OK: round-trip sem alterações");
                    } else {
                        eprintln!(
                            "WARN: round-trip preservou texto, mas normalizou bytes/encoding"
                        );
                    }
                } else {
                    eprintln!("WARN: round-trip alterou o conteúdo textual");
                }
            }
        }
        Command::Strings {
            input,
            out,
            min_len,
        } => {
            let input_bytes =
                fs::read(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let entries = extract_binary_text_entries(&input_bytes, min_len);
            fs::write(&out, serde_json::to_vec_pretty(&entries)?)?;
            eprintln!("OK: {} strings textuais prováveis", entries.len());
        }
    }
    Ok(())
}
