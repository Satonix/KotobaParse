use encoding_rs::{Encoding, SHIFT_JIS, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaParserSpec {
    pub name: String,
    pub id: String,
    pub target: Option<String>,
    pub extensions: Vec<String>,
    pub encoding: Option<String>,
    #[serde(default)]
    pub line_indexed: Option<IndexedLineSpec>,
    #[serde(default)]
    pub types: BTreeMap<String, KotobaTypeSpec>,
    #[serde(default)]
    pub skip_rules: Vec<KotobaSkipRule>,
    #[serde(default)]
    pub quote_pairs: Vec<(String, String)>,
    #[serde(default)]
    pub protect: Vec<KotobaProtectRule>,
    #[serde(default)]
    pub binary_blocks: Vec<KotobaBinaryBlockSpec>,
    #[serde(default)]
    pub states: Vec<KotobaStateSpec>,
    #[serde(default)]
    pub blocks: Vec<KotobaBlockSpec>,
    #[serde(default)]
    pub json_paths: Vec<KotobaJsonPathSpec>,
    #[serde(default)]
    pub transforms: Vec<KotobaTransformRule>,
    #[serde(default)]
    pub rebuild_strategy: Option<KotobaRebuildStrategySpec>,
    pub rules: Vec<KotobaRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedLineSpec {
    pub open: String,
    pub close: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaTypeSpec {
    pub name: String,
    #[serde(default)]
    pub patterns: Vec<String>,
    #[serde(default)]
    pub values: Vec<String>,
    #[serde(default)]
    pub trim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum KotobaSkipRule {
    Empty,
    /// Legacy alias for startswith. Kept for older parsers.
    Prefix(String),
    /// Legacy alias for unless contains. Kept for older parsers.
    Unless(String),
    StartsWith(String),
    Contains(String),
    EndsWith(String),
    Equals(String),
    UnlessStartsWith(String),
    UnlessContains(String),
    UnlessEndsWith(String),
    UnlessEquals(String),
    /// Regex-based skip. Recipe DSL exposes this as `content matching "..."`.
    Matching(String),
    Asset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum KotobaProtectRule {
    Literal(String),
    Pattern(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaBinaryBlockSpec {
    pub name: String,
    #[serde(default)]
    pub magic: Vec<u8>,
    #[serde(default = "default_binary_length")]
    pub length: String,
    #[serde(default = "default_binary_encoding")]
    pub encoding: String,
    #[serde(default = "default_binary_min_len")]
    pub min_len: usize,
    #[serde(default = "default_binary_profile")]
    pub profile: String,
}

fn default_binary_length() -> String {
    "u32le".into()
}
fn default_binary_encoding() -> String {
    "utf-8".into()
}
fn default_binary_min_len() -> usize {
    4
}
fn default_binary_profile() -> String {
    "plain".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaStateSpec {
    pub name: String,
    #[serde(default)]
    pub initial: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum KotobaCondition {
    Exists { name: String },
    NotExists { name: String },
    Equals { name: String, value: String },
    NotEquals { name: String, value: String },
    Contains { name: String, value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaSetDirective {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaTransformRule {
    pub field: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub on_extract: bool,
    #[serde(default)]
    pub on_rebuild: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaBlockSpec {
    pub name: String,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaJsonPathSpec {
    pub name: String,
    pub entries: String,
    pub text: String,
    #[serde(default)]
    pub speaker: Option<String>,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaRebuildStrategySpec {
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub on_tag_mismatch: String,
    #[serde(default)]
    pub allow_line_growth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaRule {
    pub name: String,
    /// Compatibility field used by older scaffold UI/tests.
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub source_line: Option<usize>,
    pub pattern: String,
    #[serde(default)]
    pub entry_type: Option<String>,
    #[serde(default)]
    pub text_field: Option<String>,
    #[serde(default)]
    pub speaker_field: Option<String>,
    #[serde(default)]
    pub context_field: Option<String>,
    #[serde(default)]
    pub extra_fields: BTreeMap<String, String>,
    #[serde(default)]
    pub patch_field: Option<String>,
    #[serde(default)]
    pub speaker_patch_field: Option<String>,
    #[serde(default)]
    pub skip: bool,
    #[serde(default)]
    pub remember: Vec<String>,
    #[serde(default)]
    pub forget: Vec<String>,
    #[serde(default)]
    pub when: Vec<KotobaCondition>,
    #[serde(default)]
    pub set: Vec<KotobaSetDirective>,
    /// Optional look-ahead condition used by Recipe DSL helpers such as:
    /// `speaker: content when previous is voice and next is quoted`.
    /// The pattern is matched against the next non-globally-skipped logical line.
    #[serde(default)]
    pub next_pattern: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaEntry {
    pub id: String,
    pub index: usize,
    pub kind: String,
    pub speaker: Option<String>,
    pub text: String,
    pub context: Option<String>,
    pub line: usize,
    pub rule: String,
    #[serde(default)]
    pub fields: BTreeMap<String, String>,
    #[serde(default)]
    pub patch_field: String,
    #[serde(default)]
    pub speaker_patch_field: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaExtractReport {
    pub total_entries: usize,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaRebuildReport {
    pub total_patches: usize,
    pub applied_patches: usize,
    pub skipped_patches: usize,
    pub changed_lines: Vec<usize>,
    pub warnings: Vec<String>,
    #[serde(default)]
    pub lossy_replacements: Vec<KotobaLossyReplacement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaLossyReplacement {
    pub patch_id: String,
    pub entry_id: String,
    pub entry_index: usize,
    pub field: String,
    pub encoding: String,
    pub characters: Vec<KotobaUnencodableCharacter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaUnencodableCharacter {
    pub character: String,
    pub codepoint: String,
    pub replacement_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KotobaRebuildPreview {
    pub changed: bool,
    pub report: KotobaRebuildReport,
    pub changes: Vec<KotobaLineChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaLineChange {
    pub line: usize,
    pub before: String,
    pub after: String,
    #[serde(default)]
    pub entries: Vec<KotobaChangedEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaChangedEntry {
    pub id: String,
    pub index: usize,
    pub kind: String,
    pub rule: String,
    #[serde(default)]
    pub speaker: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaDiagnostic {
    pub severity: String,
    pub message: String,
    #[serde(default)]
    pub line: Option<usize>,
    #[serde(default)]
    pub column: Option<usize>,
    #[serde(default)]
    pub rule: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaSymbol {
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub line: Option<usize>,
    #[serde(default)]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaCaptureSummary {
    pub name: String,
    #[serde(rename = "type")]
    pub capture_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaTypeSummary {
    pub name: String,
    pub patterns: Vec<String>,
    pub values: Vec<String>,
    pub trim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaRuleSummary {
    pub name: String,
    #[serde(default)]
    pub line: Option<usize>,
    #[serde(default)]
    pub entry_type: Option<String>,
    pub skip: bool,
    pub pattern: String,
    pub captures: Vec<KotobaCaptureSummary>,
    pub patch_fields: Vec<String>,
    pub remember: Vec<String>,
    pub forget: Vec<String>,
    #[serde(default)]
    pub when: Vec<KotobaCondition>,
    #[serde(default)]
    pub set: Vec<KotobaSetDirective>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaParserSummary {
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub target: Option<String>,
    pub extensions: Vec<String>,
    pub encoding: String,
    pub types: Vec<KotobaTypeSummary>,
    pub rules: Vec<KotobaRuleSummary>,
    pub protect: Vec<KotobaProtectRule>,
    pub quote_pairs: Vec<(String, String)>,
    #[serde(default)]
    pub binary_blocks: Vec<KotobaBinaryBlockSpec>,
    #[serde(default)]
    pub states: Vec<KotobaStateSpec>,
    #[serde(default)]
    pub blocks: Vec<KotobaBlockSpec>,
    #[serde(default)]
    pub json_paths: Vec<KotobaJsonPathSpec>,
    #[serde(default)]
    pub transforms: Vec<KotobaTransformRule>,
    #[serde(default)]
    pub rebuild_strategy: Option<KotobaRebuildStrategySpec>,
    pub symbols: Vec<KotobaSymbol>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaDiagnosticReport {
    pub ok: bool,
    pub diagnostics: Vec<KotobaDiagnostic>,
    #[serde(default)]
    pub summary: Option<KotobaParserSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaLanguageSpec {
    pub version: String,
    pub stability: String,
    pub goal: String,
    pub execution_model: Vec<KotobaLanguageFeature>,
    pub source_model: Vec<KotobaLanguageFeature>,
    pub extraction_model: Vec<KotobaLanguageFeature>,
    pub rebuild_model: Vec<KotobaLanguageFeature>,
    pub binary_model: Vec<KotobaLanguageFeature>,
    pub diagnostics_model: Vec<KotobaLanguageFeature>,
    pub reserved_keywords: Vec<String>,
    pub required_runtime_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaLanguageFeature {
    pub name: String,
    pub status: String,
    pub purpose: String,
    pub syntax: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaPatchInput {
    pub id: String,
    pub index: usize,
    pub source: String,
    #[serde(default)]
    pub translation: String,
    #[serde(default)]
    pub speaker_translation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaCharacterSubstitution {
    pub source: String,
    #[serde(default = "default_substitution_mode")]
    pub mode: String,
    pub target: String,
}

fn default_substitution_mode() -> String {
    "text".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KotobaBinaryString {
    pub index: usize,
    pub offset: usize,
    pub text: String,
}

#[derive(Debug, Error)]
pub enum KotobaError {
    #[error("KotobaParse inválido na linha {line}: {message}")]
    Parse { line: usize, message: String },
    #[error("Regex inválido na regra {rule}: {message}")]
    Regex { rule: String, message: String },
    #[error("Encoding inválido ({encoding}): {message}")]
    Encoding { encoding: String, message: String },
}

pub fn decode_bytes(bytes: &[u8], encoding_hint: Option<&str>) -> Result<String, KotobaError> {
    let encoding_name = encoding_hint.unwrap_or("utf-8").trim();
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8(bytes[3..].to_vec()).map_err(|e| KotobaError::Encoding {
            encoding: "utf-8-sig".into(),
            message: e.to_string(),
        });
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let (text, _, had_errors) = UTF_16LE.decode(&bytes[2..]);
        if had_errors {
            return Err(KotobaError::Encoding {
                encoding: "utf-16le".into(),
                message: "decoder reported malformed data".into(),
            });
        }
        return Ok(text.into_owned());
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let (text, _, had_errors) = UTF_16BE.decode(&bytes[2..]);
        if had_errors {
            return Err(KotobaError::Encoding {
                encoding: "utf-16be".into(),
                message: "decoder reported malformed data".into(),
            });
        }
        return Ok(text.into_owned());
    }

    let normalized = normalize_encoding_label(encoding_name);
    let Some(encoding) = encoding_for_label(&normalized) else {
        return Err(KotobaError::Encoding {
            encoding: encoding_name.to_string(),
            message: "encoding não suportado pelo runtime Rust".into(),
        });
    };
    let (text, _, had_errors) = encoding.decode(bytes);
    if had_errors && normalized == "utf-8" {
        return Err(KotobaError::Encoding {
            encoding: encoding_name.to_string(),
            message: "os bytes de entrada não são válidos para este encoding".into(),
        });
    }
    Ok(text.into_owned())
}

/// Decodes the source script from its own byte representation.
///
/// The Recipe-level `encoding` property is intentionally not consulted here:
/// it describes the rebuilt file's output encoding. BOMs and valid UTF text
/// are deterministic; legacy text falls back to a conservative CP932 versus
/// Windows-1252 distinction.
pub fn decode_source_bytes(bytes: &[u8]) -> Result<String, KotobaError> {
    let detected = detect_source_encoding(bytes);
    decode_bytes(bytes, Some(detected))
}

fn detect_source_encoding(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return "utf-8-sig";
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return "utf-16le";
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return "utf-16be";
    }
    if std::str::from_utf8(bytes).is_ok() {
        return "utf-8";
    }

    // UTF-16 without BOM is common in VN scripts. Null-byte position gives a
    // useful signal without making the output encoding part of source parsing.
    if bytes.len() >= 4 {
        let pairs = bytes.len() / 2;
        let even_nulls = bytes.iter().step_by(2).filter(|&&byte| byte == 0).count();
        let odd_nulls = bytes
            .iter()
            .skip(1)
            .step_by(2)
            .filter(|&&byte| byte == 0)
            .count();
        if odd_nulls * 2 > pairs && even_nulls * 4 < pairs {
            return "utf-16le";
        }
        if even_nulls * 2 > pairs && odd_nulls * 4 < pairs {
            return "utf-16be";
        }
    }

    let (shift_jis, _, shift_jis_errors) = SHIFT_JIS.decode(bytes);
    let windows_1252_has_c1_controls = bytes
        .iter()
        .any(|byte| matches!(byte, 0x81 | 0x8D | 0x8F | 0x90 | 0x9D));
    if !shift_jis_errors
        && (shift_jis.chars().any(is_japanese_character) || windows_1252_has_c1_controls)
    {
        return "cp932";
    }
    "windows-1252"
}

fn is_japanese_character(ch: char) -> bool {
    matches!(ch,
        '\u{3040}'..='\u{30ff}' |
        '\u{31f0}'..='\u{31ff}' |
        '\u{3400}'..='\u{4dbf}' |
        '\u{4e00}'..='\u{9fff}' |
        '\u{ff61}'..='\u{ff9f}'
    )
}

pub fn encode_text(text: &str, encoding_hint: Option<&str>) -> Result<Vec<u8>, KotobaError> {
    let encoding_name = encoding_hint.unwrap_or("utf-8").trim();
    let normalized = normalize_encoding_label(encoding_name);
    match normalized.as_str() {
        "utf-8" => Ok(text.as_bytes().to_vec()),
        "utf-8-sig" | "utf8-bom" | "utf-8-bom" => {
            let mut out = vec![0xEF, 0xBB, 0xBF];
            out.extend_from_slice(text.as_bytes());
            Ok(out)
        }
        "utf16" | "utf-16" | "utf16le" | "utf-16le" => {
            let mut out = vec![0xFF, 0xFE];
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_le_bytes());
            }
            Ok(out)
        }
        "utf16be" | "utf-16be" => {
            let mut out = vec![0xFE, 0xFF];
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_be_bytes());
            }
            Ok(out)
        }
        _ => {
            let Some(encoding) = encoding_for_label(&normalized) else {
                return Err(KotobaError::Encoding {
                    encoding: encoding_name.to_string(),
                    message: "encoding não suportado pelo runtime Rust".into(),
                });
            };
            let (bytes, _, had_errors) = encoding.encode(text);
            if had_errors {
                return Err(KotobaError::Encoding {
                    encoding: encoding_name.to_string(),
                    message: "alguns caracteres não podem ser representados neste encoding".into(),
                });
            }
            Ok(bytes.into_owned())
        }
    }
}

pub fn extract_bytes(
    bytes: &[u8],
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    if !spec.json_paths.is_empty() {
        let source = decode_source_bytes(bytes)?;
        return extract_structured_json(&source, spec);
    }
    if !spec.binary_blocks.is_empty() {
        return extract_binary_blocks(bytes, spec);
    }
    let source = decode_source_bytes(bytes)?;
    extract(&source, spec)
}

pub fn rebuild_bytes(
    bytes: &[u8],
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<Vec<u8>, KotobaError> {
    let (rebuilt, _) = rebuild_bytes_with_report(bytes, spec, patches)?;
    Ok(rebuilt)
}

pub fn rebuild_bytes_with_report(
    bytes: &[u8],
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<(Vec<u8>, KotobaRebuildReport), KotobaError> {
    rebuild_bytes_with_report_mode(bytes, spec, patches, false)
}

pub fn rebuild_bytes_with_report_mode(
    bytes: &[u8],
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
    lossy: bool,
) -> Result<(Vec<u8>, KotobaRebuildReport), KotobaError> {
    rebuild_bytes_with_report_options(bytes, spec, patches, lossy, &[])
}

pub fn rebuild_bytes_with_report_options(
    bytes: &[u8],
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
    lossy: bool,
    substitutions: &[KotobaCharacterSubstitution],
) -> Result<(Vec<u8>, KotobaRebuildReport), KotobaError> {
    if !spec.json_paths.is_empty() {
        if !substitutions.is_empty() {
            return Err(KotobaError::Encoding {
                encoding: spec.encoding.clone().unwrap_or_else(|| "json".into()),
                message: "substituições por bytes ainda não são permitidas para JSON estruturado"
                    .into(),
            });
        }
        if lossy {
            return Err(KotobaError::Encoding {
                encoding: spec.encoding.clone().unwrap_or_else(|| "json".into()),
                message: "rebuild forçado não é permitido para JSON estruturado, pois não seria possível garantir que apenas campos traduzidos fossem alterados".into(),
            });
        }
        let source = decode_source_bytes(bytes)?;
        let (rebuilt, mut report) = rebuild_structured_json(&source, spec, patches)?;
        if report.applied_patches == 0 {
            return Ok((bytes.to_vec(), report));
        }
        let encoded = encode_complete_rebuilt_text(
            &rebuilt,
            spec.encoding.as_deref(),
            bytes,
            false,
            &mut report,
        )?;
        return Ok((encoded, report));
    }
    if !spec.binary_blocks.is_empty() {
        if !substitutions.is_empty() {
            return Err(KotobaError::Encoding {
                encoding: spec.encoding.clone().unwrap_or_else(|| "binary".into()),
                message: "substituições por bytes ainda não são permitidas para payloads binários"
                    .into(),
            });
        }
        if lossy {
            return Err(KotobaError::Encoding {
                encoding: spec.encoding.clone().unwrap_or_else(|| "binary".into()),
                message: "rebuild forçado ainda não é permitido para payloads binários".into(),
            });
        }
        return rebuild_binary_blocks_with_report(bytes, spec, patches);
    }
    if patches.is_empty() {
        return Ok((bytes.to_vec(), KotobaRebuildReport::default()));
    }
    let source = decode_source_bytes(bytes)?;
    let (edits, mut report) = plan_text_rebuild(&source, spec, patches)?;
    if report.applied_patches == 0 {
        return Ok((bytes.to_vec(), report));
    }
    let rebuilt = apply_byte_preserving_edits(
        bytes,
        &source,
        spec.encoding.as_deref(),
        &edits,
        lossy,
        substitutions,
        &mut report,
    )?;
    Ok((rebuilt, report))
}

pub fn preview_rebuild_bytes(
    bytes: &[u8],
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<KotobaRebuildPreview, KotobaError> {
    if !spec.json_paths.is_empty() {
        let source = decode_source_bytes(bytes)?;
        return preview_structured_json(&source, spec, patches);
    }
    if !spec.binary_blocks.is_empty() {
        let (rebuilt, report) = rebuild_binary_blocks_with_report(bytes, spec, patches)?;
        return Ok(KotobaRebuildPreview {
            changed: rebuilt != bytes,
            report,
            changes: Vec::new(),
        });
    }
    let source = decode_source_bytes(bytes)?;
    preview_rebuild(&source, spec, patches)
}

pub fn extract_binary_ascii_strings(bytes: &[u8], min_len: usize) -> Vec<KotobaBinaryString> {
    let min_len = min_len.max(1);
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        let printable = matches!(b, 0x09 | 0x0A | 0x0D | 0x20..=0x7E);
        if printable {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(s) = start.take() {
            push_binary_string(bytes, s, i, min_len, &mut out);
        }
        i += 1;
    }
    if let Some(s) = start {
        push_binary_string(bytes, s, bytes.len(), min_len, &mut out);
    }
    out
}

pub fn extract_binary_text_entries(bytes: &[u8], min_len: usize) -> Vec<KotobaEntry> {
    extract_binary_ascii_strings(bytes, min_len)
        .into_iter()
        .filter(|item| looks_like_translatable_binary_text(&item.text))
        .enumerate()
        .map(|(index, item)| {
            let mut fields = BTreeMap::new();
            fields.insert("offset".into(), item.offset.to_string());
            KotobaEntry {
                id: format!("bin_{:08x}", item.offset),
                index,
                kind: "binary_string".into(),
                speaker: None,
                text: item.text,
                context: Some(format!("offset:{}", item.offset)),
                line: 0,
                rule: "binary_ascii_string".into(),
                fields,
                patch_field: "text".into(),
                speaker_patch_field: None,
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct KotobaBinaryRecord {
    block_name: String,
    prefix_offset: usize,
    length_offset: usize,
    payload_offset: usize,
    payload_end: usize,
    payload_len: usize,
    text: String,
}

pub fn extract_binary_blocks(
    bytes: &[u8],
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    if !spec.rules.is_empty() {
        return extract_binary_blocks_with_rules(bytes, spec);
    }
    extract_binary_blocks_legacy(bytes, spec)
}

fn extract_binary_blocks_legacy(
    bytes: &[u8],
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();

    for binary in &spec.binary_blocks {
        let records = collect_binary_records(bytes, binary, &mut warnings);
        for record in records {
            if record.text.trim().len() < binary.min_len
                || !looks_like_translatable_binary_text(&record.text)
            {
                continue;
            }

            let (clean_text, speaker, voice) =
                normalize_binary_text_for_profile(&record.text, &binary.profile);
            if clean_text.trim().is_empty() {
                continue;
            }
            let mut fields = binary_record_fields(&record);
            fields.insert("raw".into(), record.text.clone());
            if let Some(speaker) = &speaker {
                fields.insert("speaker".into(), speaker.clone());
                fields.insert("selected_speaker".into(), speaker.clone());
            }
            if let Some(voice) = &voice {
                fields.insert("voice".into(), voice.clone());
            }
            fields.insert("selected".into(), clean_text.clone());

            entries.push(KotobaEntry {
                id: format!(
                    "{}_p{:08x}",
                    sanitize_id(&binary.name),
                    record.payload_offset
                ),
                index: entries.len(),
                kind: if speaker.is_some() {
                    "dialogue".into()
                } else {
                    "narration".into()
                },
                speaker,
                text: clean_text,
                context: voice.or_else(|| Some(format!("offset:{}", record.payload_offset))),
                line: 0,
                rule: binary.name.clone(),
                fields,
                patch_field: "text".into(),
                speaker_patch_field: None,
            });
        }
    }

    Ok((
        entries.clone(),
        KotobaExtractReport {
            total_entries: entries.len(),
            warnings,
        },
    ))
}

fn extract_binary_blocks_with_rules(
    bytes: &[u8],
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    let payload_spec = spec_for_binary_payload_rules(spec);

    for binary in &spec.binary_blocks {
        let records = collect_binary_records(bytes, binary, &mut warnings);
        let mut payload_context: HashMap<String, String> = initial_context(&payload_spec);
        for record in records {
            let trimmed = record.text.trim();
            if trimmed.len() < binary.min_len {
                continue;
            }
            if !binary_record_should_feed_rules(trimmed) {
                continue;
            }

            let (mut inner_entries, inner_warnings) = extract_binary_payload_text_with_context(
                &record.text,
                &payload_spec,
                &mut payload_context,
            )?;
            for warning in inner_warnings {
                warnings.push(format!(
                    "binary {} @{}: {}",
                    binary.name, record.payload_offset, warning
                ));
            }
            update_binary_speaker_lifetime_after_payload(
                &record.text,
                &inner_entries,
                &mut payload_context,
            );
            for mut entry in inner_entries.drain(..) {
                if entry.text.trim().is_empty() {
                    continue;
                }
                let inner_id = entry.id.clone();
                let inner_index = entry.index;
                let inner_line = entry.line;
                let mut fields = binary_record_fields(&record);
                fields.insert("raw".into(), record.text.clone());
                fields.insert("inner_id".into(), inner_id.clone());
                fields.insert("inner_index".into(), inner_index.to_string());
                fields.insert("inner_line".into(), inner_line.to_string());
                for (key, value) in std::mem::take(&mut entry.fields) {
                    fields.insert(key, value);
                }
                entry.id = format!(
                    "{}_p{:08x}_{}",
                    sanitize_id(&binary.name),
                    record.payload_offset,
                    sanitize_id(&inner_id)
                );
                entry.index = entries.len();
                entry.context = entry
                    .context
                    .or_else(|| Some(format!("offset:{}", record.payload_offset)));
                entry.line = 0;
                entry.fields = fields;
                if entry.patch_field.trim().is_empty() {
                    entry.patch_field = "text".into();
                }
                entries.push(entry);
            }
        }
    }

    Ok((
        entries.clone(),
        KotobaExtractReport {
            total_entries: entries.len(),
            warnings,
        },
    ))
}

fn binary_record_should_feed_rules(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    if binary_record_is_known_control_payload(trimmed) {
        return false;
    }
    true
}

fn binary_record_is_known_control_payload(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return true;
    }

    // Speaker/voice marker records are control records, but they must still
    // pass through the flexible rule engine so they can update context for the
    // following binary text records.
    if trimmed.starts_with("//【") || trimmed.starts_with("<voice ") {
        return false;
    }

    let lowered = trimmed.to_ascii_lowercase();
    if matches!(
        trimmed,
        "main" | "TransText" | "TransAddText" | "TransChoice" | "TransLog" | "TransVoice"
    ) {
        return true;
    }
    if lowered.starts_with("media/")
        || lowered.starts_with("voice/")
        || lowered.ends_with(".nut")
        || lowered.ends_with(".ogg")
        || lowered.ends_with(".png")
        || lowered.ends_with(".bmp")
    {
        return true;
    }
    if trimmed.starts_with("' class='") || trimmed.starts_with("' src='") {
        return true;
    }
    if matches!(trimmed, "<?>" | "<K>" | "<k>" | "<I>" | "</I>") {
        return true;
    }
    false
}

fn update_binary_speaker_lifetime_after_payload(
    text: &str,
    entries: &[KotobaEntry],
    context: &mut HashMap<String, String>,
) {
    let produced_spoken_entry = entries.iter().any(|entry| {
        entry
            .speaker
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
    });
    if !produced_spoken_entry || !context.contains_key("speaker") {
        return;
    }

    let was_inside_quote = context.remove("__binary_dialogue_open").is_some();
    let quote_count = binary_dialogue_quote_count(text);

    if was_inside_quote {
        if quote_count % 2 == 0 {
            context.insert("__binary_dialogue_open".into(), "1".into());
        } else {
            context.remove("speaker");
        }
        return;
    }

    if quote_count % 2 == 1 && binary_payload_first_text_line_starts_with_quote(text) {
        context.insert("__binary_dialogue_open".into(), "1".into());
    } else {
        context.remove("speaker");
    }
}

fn binary_dialogue_quote_count(text: &str) -> usize {
    text.chars()
        .filter(|ch| matches!(ch, '"' | '“' | '”'))
        .count()
}

fn binary_payload_first_text_line_starts_with_quote(text: &str) -> bool {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("//【") || trimmed.starts_with("<voice ") {
            continue;
        }
        if binary_record_is_known_control_payload(trimmed) {
            continue;
        }
        return trimmed.starts_with('"') || trimmed.starts_with('“');
    }
    false
}

fn extract_binary_payload_text_with_context(
    source: &str,
    spec: &KotobaParserSpec,
    context: &mut HashMap<String, String>,
) -> Result<(Vec<KotobaEntry>, Vec<String>), KotobaError> {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    let source_lines: Vec<&str> = source.lines().collect();

    for (line_index, original_line) in source_lines.iter().enumerate() {
        let physical_line = line_index + 1;
        let match_line = logical_match_line(original_line, spec);
        if global_skip(match_line, spec) {
            continue;
        }

        for rule in &spec.rules {
            if !rule_next_pattern_matches(rule, line_index, &source_lines, spec)? {
                continue;
            }
            let captures = match match_rule(rule, match_line, spec)? {
                Some(captures) => Some(captures),
                None => {
                    let trimmed = match_line.trim();
                    if trimmed != match_line {
                        match_rule(rule, trimmed, spec)?
                    } else {
                        None
                    }
                }
            };
            let Some(captures) = captures else {
                continue;
            };
            if !rule_conditions_match(&rule.when, &captures, context) {
                continue;
            }

            for key in &rule.forget {
                context.remove(key);
            }
            for directive in &rule.set {
                if let Some(value) = resolve_set_value(&directive.value, &captures, context, spec) {
                    context.insert(directive.name.clone(), value);
                }
            }
            remember_rule_captures(rule, &captures, context, spec);

            if rule.skip || rule.entry_type.is_none() {
                break;
            }

            let entry_type = rule
                .entry_type
                .clone()
                .unwrap_or_else(|| "Narration".into());
            let text_field = rule.text_field.clone().unwrap_or_else(|| "text".into());
            let patch_field = rule
                .patch_field
                .clone()
                .unwrap_or_else(|| text_field.clone());

            let Some(raw_text) = captures
                .get(&text_field)
                .or_else(|| context.get(&text_field))
            else {
                warnings.push(format!(
                    "linha {}: regra {} não capturou campo de texto {}",
                    physical_line, rule.name, text_field
                ));
                break;
            };
            let source_type = capture_type_for(rule, &text_field).unwrap_or("line");
            let text = apply_extract_transforms(
                &text_field,
                &normalize_captured(raw_text, source_type, spec),
                spec,
            );
            if text.trim().is_empty() {
                break;
            }

            let speaker = rule
                .speaker_field
                .as_ref()
                .and_then(|field| resolve_runtime_field(field, &captures, context, rule, spec));
            let context_value = rule
                .context_field
                .as_ref()
                .and_then(|field| resolve_runtime_field(field, &captures, context, rule, spec));
            let mut fields = normalized_fields(&captures, spec);
            enrich_derived_fields(&mut fields);
            for (k, v) in context.iter() {
                if !k.starts_with("__") {
                    fields.entry(k.clone()).or_insert_with(|| v.clone());
                }
            }
            if let Some(s) = &speaker {
                fields.insert("selected_speaker".into(), s.clone());
            }
            fields.insert("selected".into(), text.clone());

            entries.push(KotobaEntry {
                id: format!(
                    "{}_l{:05}_{}",
                    sanitize_id(&rule.name),
                    physical_line,
                    entries.len() + 1
                ),
                index: entries.len(),
                kind: kind_to_snake(&entry_type),
                speaker,
                text,
                context: context_value,
                line: physical_line,
                rule: rule.name.clone(),
                fields,
                patch_field,
                speaker_patch_field: rule.speaker_patch_field.clone(),
            });
            break;
        }
    }

    Ok((entries, warnings))
}

fn collect_binary_records(
    bytes: &[u8],
    binary: &KotobaBinaryBlockSpec,
    warnings: &mut Vec<String>,
) -> Vec<KotobaBinaryRecord> {
    let mut records = Vec::new();
    if binary.magic.is_empty() {
        warnings.push(format!(
            "binary block {} ignorado: magic vazio",
            binary.name
        ));
        return records;
    }
    if normalize_binary_length(&binary.length) != "u32le" {
        warnings.push(format!(
            "binary block {} ignorado: length {} ainda não suportado",
            binary.name, binary.length
        ));
        return records;
    }

    let mut cursor = 0usize;
    while let Some(prefix_offset) = find_bytes(bytes, &binary.magic, cursor) {
        let length_offset = prefix_offset + binary.magic.len();
        if length_offset + 4 > bytes.len() {
            break;
        }
        let payload_len = u32::from_le_bytes([
            bytes[length_offset],
            bytes[length_offset + 1],
            bytes[length_offset + 2],
            bytes[length_offset + 3],
        ]) as usize;
        let payload_offset = length_offset + 4;
        let payload_end = payload_offset.saturating_add(payload_len);
        if payload_end > bytes.len() {
            warnings.push(format!(
                "binary block {} em offset {} ignorado: tamanho fora do arquivo",
                binary.name, prefix_offset
            ));
            cursor = prefix_offset + 1;
            continue;
        }
        let payload = &bytes[payload_offset..payload_end];
        let text = match decode_binary_payload(payload, &binary.encoding) {
            Ok(text) => text,
            Err(message) => {
                warnings.push(format!(
                    "binary block {} em offset {} ignorado: {}",
                    binary.name, prefix_offset, message
                ));
                cursor = payload_end.max(prefix_offset + 1);
                continue;
            }
        };
        records.push(KotobaBinaryRecord {
            block_name: binary.name.clone(),
            prefix_offset,
            length_offset,
            payload_offset,
            payload_end,
            payload_len,
            text,
        });
        cursor = payload_end.max(prefix_offset + 1);
    }
    records
}

fn binary_record_fields(record: &KotobaBinaryRecord) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    fields.insert("binary_block".into(), record.block_name.clone());
    fields.insert("prefix_offset".into(), record.prefix_offset.to_string());
    fields.insert("length_offset".into(), record.length_offset.to_string());
    fields.insert("payload_offset".into(), record.payload_offset.to_string());
    fields.insert("payload_end".into(), record.payload_end.to_string());
    fields.insert("length".into(), record.payload_len.to_string());
    fields
}

fn spec_for_binary_payload_rules(spec: &KotobaParserSpec) -> KotobaParserSpec {
    let mut payload_spec = spec.clone();
    payload_spec.target = None;
    payload_spec.extensions = Vec::new();
    payload_spec.encoding = Some("utf-8".into());
    payload_spec.binary_blocks = Vec::new();
    payload_spec.json_paths = Vec::new();
    payload_spec.rebuild_strategy = None;
    payload_spec
}

fn rebuild_binary_blocks_with_report(
    bytes: &[u8],
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<(Vec<u8>, KotobaRebuildReport), KotobaError> {
    let mut report = KotobaRebuildReport {
        total_patches: patches.len(),
        ..KotobaRebuildReport::default()
    };
    if patches.is_empty() {
        return Ok((bytes.to_vec(), report));
    }
    if spec.rules.is_empty() {
        report.skipped_patches = patches.len();
        report.warnings.push(
            "rebuild binário exige rules flexíveis; parser binário legado ainda é somente extração"
                .into(),
        );
        return Ok((bytes.to_vec(), report));
    }

    let (entries, extract_report) = extract_binary_blocks(bytes, spec)?;
    report.warnings.extend(extract_report.warnings);
    let mut grouped: BTreeMap<usize, Vec<(KotobaPatchInput, KotobaEntry)>> = BTreeMap::new();

    for (pos, patch) in patches.iter().enumerate() {
        let entry = if !patch.id.trim().is_empty() {
            entries.iter().find(|entry| entry.id == patch.id)
        } else {
            entries.get(patch.index).or_else(|| entries.get(pos))
        };
        let Some(entry) = entry else {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: entry binária não encontrada",
                patch_label(pos, patch)
            ));
            continue;
        };
        let Some(offset) = entry
            .fields
            .get("payload_offset")
            .and_then(|v| v.parse::<usize>().ok())
        else {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: entry binária sem payload_offset",
                patch_label(pos, patch)
            ));
            continue;
        };
        grouped
            .entry(offset)
            .or_default()
            .push((patch.clone(), entry.clone()));
    }

    let mut out = bytes.to_vec();
    let payload_spec = spec_for_binary_payload_rules(spec);
    let mut groups: Vec<(usize, Vec<(KotobaPatchInput, KotobaEntry)>)> =
        grouped.into_iter().collect();
    groups.sort_by(|a, b| b.0.cmp(&a.0));

    for (_offset, items) in groups {
        let first_entry = &items[0].1;
        let block_name = first_entry
            .fields
            .get("binary_block")
            .cloned()
            .unwrap_or_default();
        let Some(binary) = spec
            .binary_blocks
            .iter()
            .find(|block| block.name == block_name)
        else {
            report.skipped_patches += items.len();
            report.warnings.push(format!(
                "payload binário ignorado: bloco {} não encontrado",
                block_name
            ));
            continue;
        };
        let Some(length_offset) = first_entry
            .fields
            .get("length_offset")
            .and_then(|v| v.parse::<usize>().ok())
        else {
            report.skipped_patches += items.len();
            continue;
        };
        let Some(payload_offset) = first_entry
            .fields
            .get("payload_offset")
            .and_then(|v| v.parse::<usize>().ok())
        else {
            report.skipped_patches += items.len();
            continue;
        };
        let Some(payload_end) = first_entry
            .fields
            .get("payload_end")
            .and_then(|v| v.parse::<usize>().ok())
        else {
            report.skipped_patches += items.len();
            continue;
        };
        if length_offset + 4 > out.len() || payload_offset > payload_end || payload_end > out.len()
        {
            report.skipped_patches += items.len();
            report.warnings.push(format!(
                "payload binário @{} ignorado: offsets inválidos",
                payload_offset
            ));
            continue;
        }
        let payload_text =
            match decode_binary_payload(&out[payload_offset..payload_end], &binary.encoding) {
                Ok(text) => text,
                Err(message) => {
                    report.skipped_patches += items.len();
                    report.warnings.push(format!(
                        "payload binário @{} ignorado: {}",
                        payload_offset, message
                    ));
                    continue;
                }
            };
        let mut local_patches = Vec::new();
        for (patch, entry) in items {
            let inner_id = entry.fields.get("inner_id").cloned().unwrap_or_default();
            let inner_index = entry
                .fields
                .get("inner_index")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(entry.index);
            local_patches.push(KotobaPatchInput {
                id: inner_id,
                index: inner_index,
                source: patch.source,
                translation: patch.translation,
                speaker_translation: patch.speaker_translation,
            });
        }
        let (rebuilt_text, local_report) =
            rebuild_with_report(&payload_text, &payload_spec, &local_patches)?;
        report.warnings.extend(
            local_report
                .warnings
                .into_iter()
                .map(|warning| format!("binary @{}: {}", payload_offset, warning)),
        );
        if local_report.applied_patches == 0 {
            report.skipped_patches += local_report.skipped_patches.max(local_patches.len());
            continue;
        }
        let rebuilt_payload = encode_binary_payload(&rebuilt_text, &binary.encoding)?;
        if rebuilt_payload.len() > u32::MAX as usize {
            report.skipped_patches += local_report.applied_patches;
            report.warnings.push(format!(
                "payload binário @{} ignorado: texto reconstruído excede u32",
                payload_offset
            ));
            continue;
        }
        let len_bytes = (rebuilt_payload.len() as u32).to_le_bytes();
        out[length_offset..length_offset + 4].copy_from_slice(&len_bytes);
        out.splice(payload_offset..payload_end, rebuilt_payload.into_iter());
        report.applied_patches += local_report.applied_patches;
        report.skipped_patches += local_report.skipped_patches;
        report.changed_lines.push(payload_offset);
    }

    report.changed_lines.sort_unstable();
    report.changed_lines.dedup();
    Ok((out, report))
}

fn encode_binary_payload(text: &str, encoding: &str) -> Result<Vec<u8>, KotobaError> {
    let encoding_name = encoding.trim();
    let normalized = normalize_encoding_label(encoding_name);
    match normalized.as_str() {
        "utf8" | "utf-8" => Ok(text.as_bytes().to_vec()),
        "utf16" | "utf-16" | "utf16le" | "utf-16le" => {
            let mut out = Vec::new();
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_le_bytes());
            }
            Ok(out)
        }
        "utf16be" | "utf-16be" => {
            let mut out = Vec::new();
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_be_bytes());
            }
            Ok(out)
        }
        _ => {
            let Some(encoding) = encoding_for_label(&normalized) else {
                return Err(KotobaError::Encoding {
                    encoding: encoding_name.to_string(),
                    message: "encoding não suportado pelo runtime Rust".into(),
                });
            };
            let (bytes, _, had_errors) = encoding.encode(text);
            if had_errors {
                return Err(KotobaError::Encoding {
                    encoding: encoding_name.to_string(),
                    message: "alguns caracteres não podem ser representados neste encoding".into(),
                });
            }
            Ok(bytes.into_owned())
        }
    }
}

pub fn language_spec() -> KotobaLanguageSpec {
    fn feature(
        name: &str,
        status: &str,
        purpose: &str,
        syntax: &[&str],
        notes: &[&str],
    ) -> KotobaLanguageFeature {
        KotobaLanguageFeature {
            name: name.into(),
            status: status.into(),
            purpose: purpose.into(),
            syntax: syntax.iter().map(|s| (*s).into()).collect(),
            notes: notes.iter().map(|s| (*s).into()).collect(),
        }
    }

    KotobaLanguageSpec {
        version: "0.3-recipe".into(),
        stability: "alpha-strict".into(),
        goal: "Uma única Recipe canônica para extração, diagnóstico, preview e reinjeção previsível de scripts de visual novel.".into(),
        execution_model: vec![
            feature("recipe", "implemented", "Define uma unidade compilável KotobaParse.", &["parser Nome:", "    file \".ks\"", "    encoding cp932"], &["Qualquer cabeçalho ou sintaxe anterior é erro."]),
            feature("metadata", "implemented", "Declara extensão, encoding de saída e tipos reutilizáveis.", &["file \".sc\"", "encoding cp932", "type VoiceId = matches \"...\""], &[]),
            feature("strict_commands", "implemented", "Garante uma única grafia por operação.", &["when starts with", "capture text as quoted", "remember speaker", "save as Dialogue", "patch text", "skip"], &["Comandos desconhecidos, aliases antigos e comandos sem efeito são erros."]),
        ],
        source_model: vec![
            feature("line_stream", "implemented", "Processa scripts linha a linha preservando numeração física.", &["read:", "    records as lines"], &[]),
            feature("numbered_lines", "implemented", "Remove prefixos numéricos delimitados sem perder a linha original.", &["numbered lines:", "    id between \"<\" and \">\""], &[]),
            feature("blocks", "implemented", "Agrupa várias linhas em records estruturais.", &["read:", "    records as blocks", "    block starts with \"text = {\"", "    block ends when braces close"], &[]),
            feature("segmented_lines", "implemented", "Mapeia campos multilíngues separados.", &["records as segmented lines", "fields \"en\", \"ja\"", "source field \"en\""], &[]),
        ],
        extraction_model: vec![
            feature("semantic_blocks", "implemented", "Expressa extração e estado sem expor uma regra genérica.", &["dialogue Main:", "text SceneTitle:", "choice Menu:", "ignore Command:"], &["O nome é opcional e permite repetir o mesmo bloco semântico para formatos diferentes."]),
            feature("semantic_conditions", "implemented", "Combina condições e capturas flexíveis dentro dos blocos semânticos.", &["when matches \"...\"", "when format \"...\"", "capture text as field 3 separated by tab", "remember speaker"], &[]),
            feature("legacy_rule", "compatibility", "Lê Recipes 0.3.0-alpha.2 que ainda possuem `rule Nome:`.", &["rule Nome:"], &["Não é sugerido pelo autocomplete nem necessário em novos parsers."]),
            feature("typed_captures", "implemented", "Valida capturas por tipos reutilizáveis.", &["type Speaker = matches \"...\"", "when format \"<speaker:Speaker>: <text:rest>\""], &[]),
            feature("skip", "implemented", "Ignora records globais ou padrões ordenados.", &["ignore:", "    empty", "ignore Command:", "    when matches \"...\""], &[]),
        ],
        rebuild_model: vec![
            feature("field_patch", "implemented", "Reinjeta somente o campo capturado, preservando prefixos, aspas e comandos.", &["patch text", "patch speaker"], &[]),
            feature("preview", "implemented", "Mostra mudanças antes de gravar o arquivo reconstruído.", &["kotoba preview parser.kotoba input patches.json preview.json"], &[]),
            feature("report", "implemented", "Informa patches aplicados, ignorados, linhas alteradas e warnings.", &["rebuild_with_report(...)"], &[]),
            feature("roundtrip", "implemented", "Valida que sem patches o arquivo não muda.", &["kotoba roundtrip parser.kotoba input"], &[]),
            feature("tag_integrity", "implemented", "Valida tokens protegidos entre origem e tradução.", &["protect:", "    between \"[\" and \"]\"", "    matches \"@[^\\s]+\""], &[]),
        ],
        binary_model: vec![
            feature("binary_records", "implemented", "Extrai e reinjeta payloads binários com magic, length e encoding.", &["read:", "    records as binary", "    magic \"10 00 00 08\"", "    length u32le", "    encoding utf8"], &[]),
            feature("binary_strings", "implemented", "Varredura de strings textuais prováveis em binários.", &["kotoba strings input.bin out.json"], &["Ferramenta auxiliar, não parser final."]),
            feature("offset_entries", "experimental", "Entries binárias carregam offsets/tamanho no campo fields.", &["fields.prefix_offset", "fields.payload_offset", "fields.length"], &[]),
            feature("binary_rebuild", "implemented", "Reinjeta payloads e atualiza seu tamanho u32le.", &["patch text"], &[]),
        ],
        diagnostics_model: vec![
            feature("diagnose", "implemented", "Retorna erros/warnings em JSON para editor.", &["kotoba diagnose parser.kotoba", "kotoba check parser.kotoba --json"], &[]),
            feature("summary", "implemented", "Retorna outline de parser, regras, captures e símbolos.", &["kotoba summary parser.kotoba"], &[]),
            feature("language_spec", "implemented", "Retorna esta matriz de recursos em JSON para UI/documentação.", &["kotoba language-spec"], &[]),
            feature("strict_syntax", "implemented", "Rejeita qualquer sintaxe, alias ou comando removido.", &["kotoba check parser.kotoba"], &[]),
        ],
        reserved_keywords: vec![
            "parser", "file", "encoding", "type", "read", "numbered", "quotes", "protect", "ignore", "voice", "speaker", "dialogue", "text", "choice", "rule", "when", "capture", "save", "patch", "skip", "remember", "forget", "matches", "like",
        ].into_iter().map(String::from).collect(),
        required_runtime_commands: vec![
            "check".into(), "diagnose".into(), "summary".into(), "language-spec".into(), "extract".into(), "preview".into(), "rebuild".into(), "roundtrip".into(), "strings".into(),
        ],
    }
}

pub fn parse_header_only(source: &str) -> KotobaParserSpec {
    parse_source(source).unwrap_or_default()
}

pub fn diagnose_source(source: &str) -> KotobaDiagnosticReport {
    match parse_source(source) {
        Ok(spec) => {
            let diagnostics = validate_spec(&spec);
            let ok = !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.severity == "error");
            KotobaDiagnosticReport {
                ok,
                diagnostics,
                summary: Some(summarize_spec(&spec)),
            }
        }
        Err(error) => KotobaDiagnosticReport {
            ok: false,
            diagnostics: vec![diagnostic_from_error(error)],
            summary: None,
        },
    }
}

pub fn summarize_source(source: &str) -> Result<KotobaParserSummary, KotobaError> {
    let spec = parse_source(source)?;
    Ok(summarize_spec(&spec))
}

pub fn summarize_spec(spec: &KotobaParserSpec) -> KotobaParserSummary {
    let mut types = Vec::new();
    let mut rules = Vec::new();
    let mut symbols = Vec::new();
    symbols.push(KotobaSymbol {
        kind: "parser".into(),
        name: spec.name.clone(),
        line: None,
        detail: spec
            .target
            .clone()
            .or_else(|| spec.extensions.first().cloned()),
    });

    for (name, typ) in &spec.types {
        let summary = KotobaTypeSummary {
            name: name.clone(),
            patterns: typ.patterns.clone(),
            values: typ.values.clone(),
            trim: typ.trim,
        };
        let mut parts = Vec::new();
        if !summary.patterns.is_empty() {
            parts.push(format!("{} pattern(s)", summary.patterns.len()));
        }
        if !summary.values.is_empty() {
            parts.push(format!("{} value(s)", summary.values.len()));
        }
        if summary.trim {
            parts.push("trim".into());
        }
        symbols.push(KotobaSymbol {
            kind: "type".into(),
            name: name.clone(),
            line: None,
            detail: Some(parts.join(", ")).filter(|s| !s.is_empty()),
        });
        types.push(summary);
    }

    for rule in &spec.rules {
        let captures = captures_from_pattern(&rule.pattern)
            .into_iter()
            .map(|(name, capture_type)| KotobaCaptureSummary { name, capture_type })
            .collect::<Vec<_>>();
        let mut patch_fields = Vec::new();
        if let Some(field) = &rule.patch_field {
            patch_fields.push(field.clone());
        }
        if let Some(field) = &rule.speaker_patch_field {
            patch_fields.push(field.clone());
        }
        rules.push(KotobaRuleSummary {
            name: rule.name.clone(),
            line: rule.source_line,
            entry_type: rule.entry_type.clone(),
            skip: rule.skip,
            pattern: rule.pattern.clone(),
            captures,
            patch_fields,
            remember: rule.remember.clone(),
            forget: rule.forget.clone(),
            when: rule.when.clone(),
            set: rule.set.clone(),
        });
        let detail = if rule.skip {
            Some("skip".into())
        } else {
            rule.entry_type.clone()
        };
        symbols.push(KotobaSymbol {
            kind: "rule".into(),
            name: rule.name.clone(),
            line: rule.source_line,
            detail,
        });
    }

    for binary in &spec.binary_blocks {
        symbols.push(KotobaSymbol {
            kind: "binary".into(),
            name: binary.name.clone(),
            line: None,
            detail: Some(binary.profile.clone()),
        });
    }
    for state in &spec.states {
        symbols.push(KotobaSymbol {
            kind: "state".into(),
            name: state.name.clone(),
            line: None,
            detail: state.initial.clone(),
        });
    }
    for block in &spec.blocks {
        symbols.push(KotobaSymbol {
            kind: "block".into(),
            name: block.name.clone(),
            line: None,
            detail: Some(format!("{} -> {}", block.start, block.end)),
        });
    }
    for json in &spec.json_paths {
        symbols.push(KotobaSymbol {
            kind: "json".into(),
            name: json.name.clone(),
            line: None,
            detail: Some(json.entries.clone()),
        });
    }
    for transform in &spec.transforms {
        symbols.push(KotobaSymbol {
            kind: "transform".into(),
            name: transform.field.clone(),
            line: None,
            detail: Some(format!("{} -> {}", transform.from, transform.to)),
        });
    }

    KotobaParserSummary {
        name: spec.name.clone(),
        id: spec.id.clone(),
        target: spec.target.clone(),
        extensions: spec.extensions.clone(),
        encoding: spec.encoding.clone().unwrap_or_else(|| "utf-8".into()),
        types,
        rules,
        protect: spec.protect.clone(),
        quote_pairs: spec.quote_pairs.clone(),
        binary_blocks: spec.binary_blocks.clone(),
        states: spec.states.clone(),
        blocks: spec.blocks.clone(),
        json_paths: spec.json_paths.clone(),
        transforms: spec.transforms.clone(),
        rebuild_strategy: spec.rebuild_strategy.clone(),
        symbols,
    }
}

pub fn parse_source(source: &str) -> Result<KotobaParserSpec, KotobaError> {
    if !looks_like_kotoba_recipe(source) {
        let line = source
            .lines()
            .position(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|index| index + 1)
            .unwrap_or(1);
        return Err(KotobaError::Parse {
            line,
            message: "sintaxe removida: use somente a Recipe canônica iniciada por `parser Nome:`"
                .into(),
        });
    }
    parse_recipe_source(source)
}

fn logical_match_line<'a>(line: &'a str, spec: &KotobaParserSpec) -> &'a str {
    if let Some(indexed) = &spec.line_indexed {
        if let Some((_idx, payload)) = strip_index_prefix(line, indexed) {
            return payload;
        }
    }
    line
}

fn rule_next_pattern_matches(
    rule: &KotobaRule,
    line_index: usize,
    source_lines: &[&str],
    spec: &KotobaParserSpec,
) -> Result<bool, KotobaError> {
    let Some(pattern) = rule.next_pattern.as_deref() else {
        return Ok(true);
    };
    let probe_rule = KotobaRule {
        name: format!("{}_next", rule.name),
        pattern: pattern.to_string(),
        ..KotobaRule::default()
    };
    for next in source_lines.iter().skip(line_index + 1) {
        let next_line = logical_match_line(next, spec);
        if global_skip(next_line, spec) {
            continue;
        }
        if match_rule(&probe_rule, next_line, spec)?.is_some() {
            return Ok(true);
        }
        let trimmed = next_line.trim();
        if trimmed != next_line && match_rule(&probe_rule, trimmed, spec)?.is_some() {
            return Ok(true);
        }
        return Ok(false);
    }
    Ok(false)
}

pub fn extract(
    source: &str,
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    if !spec.json_paths.is_empty() {
        return extract_structured_json(source, spec);
    }
    if !spec.blocks.is_empty() {
        let (entries, report) = extract_block_entries(source, spec)?;
        if !entries.is_empty() {
            return Ok((entries, report));
        }
    }
    let mut entries = Vec::new();
    let mut context: HashMap<String, String> = initial_context(spec);
    let mut warnings = Vec::new();

    let source_lines: Vec<&str> = source.lines().collect();
    for (line_index, original_line) in source_lines.iter().enumerate() {
        let physical_line = line_index + 1;
        let match_line = logical_match_line(original_line, spec);
        if global_skip(match_line, spec) {
            continue;
        }

        let mut matched = false;
        for rule in &spec.rules {
            if !rule_next_pattern_matches(rule, line_index, &source_lines, spec)? {
                continue;
            }
            let captures = match match_rule(rule, match_line, spec)? {
                Some(captures) => Some(captures),
                None => {
                    let trimmed = match_line.trim();
                    if trimmed != match_line {
                        match_rule(rule, trimmed, spec)?
                    } else {
                        None
                    }
                }
            };
            let Some(captures) = captures else {
                continue;
            };
            if !rule_conditions_match(&rule.when, &captures, &context) {
                continue;
            }
            matched = true;

            for key in &rule.forget {
                context.remove(key);
            }
            for directive in &rule.set {
                if let Some(value) = resolve_set_value(&directive.value, &captures, &context, spec)
                {
                    context.insert(directive.name.clone(), value);
                }
            }
            remember_rule_captures(rule, &captures, &mut context, spec);

            if rule.skip || rule.entry_type.is_none() {
                break;
            }

            let entry_type = rule
                .entry_type
                .clone()
                .unwrap_or_else(|| "Narration".into());
            let text_field = rule.text_field.clone().unwrap_or_else(|| "text".into());
            let speaker_field = rule.speaker_field.clone();
            let patch_field = rule
                .patch_field
                .clone()
                .unwrap_or_else(|| text_field.clone());

            if entry_type == "ChoiceGroup" {
                if let Some(cell) = captures.get(&text_field) {
                    let choices = split_choice_cell(&normalize_captured(cell, "cell", spec));
                    let mut choice_no = 0usize;
                    for (choice_key, choice_text) in choices {
                        let id = format!(
                            "{}_l{:05}_{}",
                            sanitize_id(&rule.name),
                            physical_line,
                            sanitize_id(&choice_key)
                        );
                        let mut fields = normalized_fields(&captures, spec);
                        fields.insert("choice_key".into(), choice_key.clone());
                        fields.insert("selected".into(), choice_text.clone());
                        fields.insert(text_field.clone(), choice_text.clone());
                        entries.push(KotobaEntry {
                            id,
                            index: entries.len(),
                            kind: "choice".into(),
                            speaker: None,
                            text: choice_text,
                            context: Some(choice_key),
                            line: physical_line,
                            rule: rule.name.clone(),
                            fields,
                            patch_field: patch_field.clone(),
                            speaker_patch_field: rule.speaker_patch_field.clone(),
                        });
                        choice_no += 1;
                    }
                    if choice_no == 0 {
                        warnings.push(format!(
                            "linha {}: ChoiceGroup não continha opções reconhecíveis",
                            physical_line
                        ));
                    }
                }
                break;
            }

            let Some(raw_text) = captures
                .get(&text_field)
                .or_else(|| context.get(&text_field))
            else {
                warnings.push(format!(
                    "linha {}: regra {} não capturou campo de texto {}",
                    physical_line, rule.name, text_field
                ));
                break;
            };
            let source_type = capture_type_for(rule, &text_field).unwrap_or("line");
            let text = apply_extract_transforms(
                &text_field,
                &normalize_captured(raw_text, source_type, spec),
                spec,
            );
            if text.trim().is_empty() {
                break;
            }

            let speaker = speaker_field
                .as_ref()
                .and_then(|field| resolve_runtime_field(field, &captures, &context, rule, spec));
            let context_value = rule
                .context_field
                .as_ref()
                .and_then(|field| resolve_runtime_field(field, &captures, &context, rule, spec));
            let mut fields = normalized_fields(&captures, spec);
            enrich_derived_fields(&mut fields);
            for (k, v) in &context {
                fields.entry(k.clone()).or_insert_with(|| v.clone());
            }
            if let Some(s) = &speaker {
                fields.insert("selected_speaker".into(), s.clone());
            }
            fields.insert("selected".into(), text.clone());

            entries.push(KotobaEntry {
                id: format!(
                    "{}_l{:05}_{}",
                    sanitize_id(&rule.name),
                    physical_line,
                    entries.len() + 1
                ),
                index: entries.len(),
                kind: kind_to_snake(&entry_type),
                speaker,
                text,
                context: context_value,
                line: physical_line,
                rule: rule.name.clone(),
                fields,
                patch_field,
                speaker_patch_field: rule.speaker_patch_field.clone(),
            });
            break;
        }
        let _ = matched;
    }

    let report = KotobaExtractReport {
        total_entries: entries.len(),
        warnings,
    };
    Ok((entries, report))
}

pub fn rebuild(
    source: &str,
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<String, KotobaError> {
    let (rebuilt, _) = rebuild_with_report(source, spec, patches)?;
    Ok(rebuilt)
}

pub fn rebuild_with_report(
    source: &str,
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<(String, KotobaRebuildReport), KotobaError> {
    if !spec.json_paths.is_empty() {
        return rebuild_structured_json(source, spec, patches);
    }
    let (mut edits, report) = plan_text_rebuild(source, spec, patches)?;
    edits.sort_by(|a, b| b.start.cmp(&a.start).then_with(|| b.end.cmp(&a.end)));
    let mut rebuilt = source.to_string();
    for edit in edits {
        rebuilt.replace_range(edit.start..edit.end, &edit.replacement);
    }
    Ok((rebuilt, report))
}

#[derive(Debug, Clone)]
struct KotobaTextEdit {
    start: usize,
    end: usize,
    replacement: String,
    patch_id: String,
    entry_id: String,
    entry_index: usize,
    field: String,
}

fn plan_text_rebuild(
    source: &str,
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<(Vec<KotobaTextEdit>, KotobaRebuildReport), KotobaError> {
    let (current, _) = extract(source, spec)?;
    let mut by_id: HashMap<String, &KotobaEntry> = HashMap::new();
    for entry in &current {
        by_id.insert(entry.id.clone(), entry);
    }

    let mut report = KotobaRebuildReport {
        total_patches: patches.len(),
        ..KotobaRebuildReport::default()
    };
    let mut changed_lines = BTreeSet::new();
    let lines: Vec<String> = split_preserving_newline(source);
    let mut line_offsets = Vec::with_capacity(lines.len());
    let mut source_offset = 0usize;
    for line in &lines {
        line_offsets.push(source_offset);
        source_offset += line.len();
    }
    let mut edits = Vec::new();

    for (pos, patch) in patches.iter().enumerate() {
        if patch.translation.trim().is_empty() && patch.speaker_translation.trim().is_empty() {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: tradução vazia",
                patch_label(pos, patch)
            ));
            continue;
        }

        let entry = if !patch.id.trim().is_empty() {
            by_id.get(&patch.id).copied()
        } else {
            current.get(patch.index)
        }
        .or_else(|| current.get(pos));

        let Some(entry) = entry else {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: entry não encontrada",
                patch_label(pos, patch)
            ));
            continue;
        };

        if !patch.source.trim().is_empty() && patch.source != entry.text {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: source divergente para {}",
                patch_label(pos, patch),
                entry.id
            ));
            continue;
        }

        if entry.line == 0 || entry.line > lines.len() {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: linha inválida {}",
                patch_label(pos, patch),
                entry.line
            ));
            continue;
        }

        let mut applied_this_patch = false;
        let patch_id = patch_label(pos, patch);

        if !patch.translation.trim().is_empty() {
            if let Some(warning) =
                protected_token_warning(spec, &entry.text, &patch.translation, &entry.id)
            {
                report.warnings.push(warning);
            }
            if entry.fields.contains_key("block_start") {
                match plan_block_entry_field_edit(
                    &lines,
                    &line_offsets,
                    spec,
                    entry,
                    &entry.patch_field,
                    &patch.translation,
                    &patch_id,
                ) {
                    Ok(Some(edit)) => {
                        edits.push(edit);
                        applied_this_patch = true;
                    }
                    Ok(None) => report.warnings.push(format!(
                        "patch {}: campo de bloco não encontrado em {}",
                        patch_id, entry.id
                    )),
                    Err(error) => return Err(error),
                }
            } else {
                match plan_line_entry_field_edit(
                    &lines[entry.line - 1],
                    line_offsets[entry.line - 1],
                    spec,
                    entry,
                    &entry.patch_field,
                    &patch.translation,
                    &patch_id,
                ) {
                    Ok(Some(edit)) => {
                        edits.push(edit);
                        applied_this_patch = true;
                    }
                    Ok(None) => report.warnings.push(format!(
                        "patch {}: campo de texto não encontrado em {}",
                        patch_id, entry.id
                    )),
                    Err(error) => return Err(error),
                }
            }
        }

        if !patch.speaker_translation.trim().is_empty() {
            if let Some(field) = entry.speaker_patch_field.as_deref() {
                if entry.fields.contains_key("block_start") {
                    match plan_block_entry_field_edit(
                        &lines,
                        &line_offsets,
                        spec,
                        entry,
                        field,
                        &patch.speaker_translation,
                        &patch_id,
                    ) {
                        Ok(Some(edit)) => {
                            edits.push(edit);
                            applied_this_patch = true;
                        }
                        Ok(None) => report.warnings.push(format!(
                            "patch {}: campo de speaker em bloco não encontrado em {}",
                            patch_id, entry.id
                        )),
                        Err(error) => return Err(error),
                    }
                } else {
                    match plan_line_entry_field_edit(
                        &lines[entry.line - 1],
                        line_offsets[entry.line - 1],
                        spec,
                        entry,
                        field,
                        &patch.speaker_translation,
                        &patch_id,
                    ) {
                        Ok(Some(edit)) => {
                            edits.push(edit);
                            applied_this_patch = true;
                        }
                        Ok(None) => report.warnings.push(format!(
                            "patch {}: campo de speaker não encontrado em {}",
                            patch_id, entry.id
                        )),
                        Err(error) => return Err(error),
                    }
                }
            } else if let Some(speaker) = &entry.speaker {
                let line = &lines[entry.line - 1];
                if let Some(start) = line_body(line).find(speaker) {
                    edits.push(KotobaTextEdit {
                        start: line_offsets[entry.line - 1] + start,
                        end: line_offsets[entry.line - 1] + start + speaker.len(),
                        replacement: patch.speaker_translation.clone(),
                        patch_id: patch_id.clone(),
                        entry_id: entry.id.clone(),
                        entry_index: entry.index,
                        field: "speaker".into(),
                    });
                    applied_this_patch = true;
                } else {
                    report.warnings.push(format!(
                        "patch {}: speaker original não encontrado em {}",
                        patch_id, entry.id
                    ));
                }
            }
        }

        if applied_this_patch {
            report.applied_patches += 1;
            changed_lines.insert(entry.line);
        } else {
            report.skipped_patches += 1;
        }
    }

    edits.sort_by_key(|edit| (edit.start, edit.end));
    for pair in edits.windows(2) {
        if pair[0].end > pair[1].start {
            return Err(KotobaError::Parse {
                line: current
                    .iter()
                    .find(|entry| entry.id == pair[1].entry_id)
                    .map(|entry| entry.line)
                    .unwrap_or(0),
                message: format!(
                    "patches {} e {} tentam alterar intervalos sobrepostos",
                    pair[0].patch_id, pair[1].patch_id
                ),
            });
        }
    }
    report.changed_lines = changed_lines.into_iter().collect();
    Ok((edits, report))
}

pub fn preview_rebuild(
    source: &str,
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<KotobaRebuildPreview, KotobaError> {
    let (entries, _) = extract(source, spec)?;
    let (rebuilt, report) = rebuild_with_report(source, spec, patches)?;
    let before_lines = split_preserving_newline(source);
    let after_lines = split_preserving_newline(&rebuilt);
    let mut changes = Vec::new();

    for line_no in &report.changed_lines {
        let before = before_lines
            .get(line_no.saturating_sub(1))
            .cloned()
            .unwrap_or_default();
        let after = after_lines
            .get(line_no.saturating_sub(1))
            .cloned()
            .unwrap_or_default();
        changes.push(KotobaLineChange {
            line: *line_no,
            before,
            after,
            entries: entries
                .iter()
                .filter(|entry| entry.line == *line_no)
                .map(changed_entry_from_entry)
                .collect(),
        });
    }

    Ok(KotobaRebuildPreview {
        changed: !changes.is_empty(),
        report,
        changes,
    })
}

fn initial_context(spec: &KotobaParserSpec) -> HashMap<String, String> {
    let mut context = HashMap::new();
    for state in &spec.states {
        if let Some(value) = &state.initial {
            context.insert(state.name.clone(), value.clone());
        }
    }
    context
}

fn lookup_runtime_value<'a>(
    name: &str,
    captures: &'a BTreeMap<String, String>,
    context: &'a HashMap<String, String>,
) -> Option<&'a String> {
    captures.get(name).or_else(|| context.get(name))
}

fn remember_rule_captures(
    rule: &KotobaRule,
    captures: &BTreeMap<String, String>,
    context: &mut HashMap<String, String>,
    spec: &KotobaParserSpec,
) {
    for key in &rule.remember {
        if let Some(value) = captures.get(key).filter(|s| !s.trim().is_empty()) {
            let typ = capture_type_for(rule, key).unwrap_or("line");
            context.insert(key.clone(), normalize_captured(value, typ, spec));
        }
    }

    // A voice section may provide both the file id and its explicit character
    // attribute (for example `file="z000100010", ch="c001"`). Preserve that
    // relationship instead of trying to infer the speaker from the file id.
    // A newly remembered voice without an explicit speaker also clears the
    // previous association so it cannot leak into the next dialogue.
    let remembers_voice = rule.remember.iter().any(|key| key == "voice");
    let captured_voice = captures
        .get("voice")
        .filter(|value| !value.trim().is_empty());
    if remembers_voice && captured_voice.is_some() {
        context.remove("voice.speaker");
        if rule.remember.iter().any(|key| key == "speaker") {
            if let Some(speaker) = captures
                .get("speaker")
                .filter(|value| !value.trim().is_empty())
            {
                let typ = capture_type_for(rule, "speaker").unwrap_or("line");
                context.insert(
                    "voice.speaker".into(),
                    normalize_captured(speaker, typ, spec),
                );
            }
        }
    }
}

fn resolve_runtime_field(
    field: &str,
    captures: &BTreeMap<String, String>,
    context: &HashMap<String, String>,
    rule: &KotobaRule,
    spec: &KotobaParserSpec,
) -> Option<String> {
    // Dotted fields can be materialized explicitly in the runtime context.
    // Prefer that exact association before falling back to derivation from the
    // base value (such as deriving a speaker from a conventional voice id).
    if let Some(value) = captures.get(field).or_else(|| context.get(field)) {
        let typ = capture_type_for(rule, field).unwrap_or("line");
        return Some(apply_extract_transforms(
            field,
            &normalize_captured(value, typ, spec),
            spec,
        ));
    }
    if let Some((base, member)) = field.split_once('.') {
        let base_value = captures.get(base).or_else(|| context.get(base))?;
        let value = derive_runtime_member(base_value, member)?;
        return Some(apply_extract_transforms(field, &value, spec));
    }
    captures.get(field).or_else(|| context.get(field)).map(|v| {
        let typ = capture_type_for(rule, field).unwrap_or("line");
        apply_extract_transforms(field, &normalize_captured(v, typ, spec), spec)
    })
}

fn derive_runtime_member(value: &str, member: &str) -> Option<String> {
    match member {
        "speaker" => derive_voice_speaker(value),
        "value" => Some(value.to_string()),
        _ => None,
    }
}

fn derive_voice_speaker(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let parts: Vec<&str> = trimmed.split('-').collect();
    if parts.len() >= 3 && !parts[1].trim().is_empty() {
        return Some(parts[1].trim().to_string());
    }
    // Common numbered-script voice ids: yuki_000001, taku_000012, yuki_000002_a.
    let re = Regex::new(r"^([A-Za-z][A-Za-z0-9]*)_[0-9]{4,}(?:_[A-Za-z0-9_]+)?$").ok()?;
    re.captures(trimmed)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

fn enrich_derived_fields(fields: &mut BTreeMap<String, String>) {
    if let Some(voice) = fields.get("voice").cloned() {
        if let Some(speaker) = derive_voice_speaker(&voice) {
            fields.entry("voice.speaker".into()).or_insert(speaker);
        }
    }
}

fn rule_conditions_match(
    conditions: &[KotobaCondition],
    captures: &BTreeMap<String, String>,
    context: &HashMap<String, String>,
) -> bool {
    conditions.iter().all(|condition| match condition {
        KotobaCondition::Exists { name } => lookup_runtime_value(name, captures, context)
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false),
        KotobaCondition::NotExists { name } => lookup_runtime_value(name, captures, context)
            .map(|v| v.trim().is_empty())
            .unwrap_or(true),
        KotobaCondition::Equals { name, value } => lookup_runtime_value(name, captures, context)
            .map(|v| v == value)
            .unwrap_or(false),
        KotobaCondition::NotEquals { name, value } => lookup_runtime_value(name, captures, context)
            .map(|v| v != value)
            .unwrap_or(true),
        KotobaCondition::Contains { name, value } => lookup_runtime_value(name, captures, context)
            .map(|v| v.contains(value))
            .unwrap_or(false),
    })
}

fn resolve_set_value(
    value: &str,
    captures: &BTreeMap<String, String>,
    context: &HashMap<String, String>,
    spec: &KotobaParserSpec,
) -> Option<String> {
    let value = value.trim();
    if let Some(inner) = value
        .strip_prefix("capture(")
        .and_then(|v| v.strip_suffix(')'))
    {
        return captures
            .get(inner.trim())
            .map(|v| normalize_captured(v, "line", spec));
    }
    if let Some(inner) = value
        .strip_prefix("state(")
        .and_then(|v| v.strip_suffix(')'))
    {
        return context.get(inner.trim()).cloned();
    }
    if let Some(v) = captures.get(value) {
        return Some(normalize_captured(v, "line", spec));
    }
    if let Some(v) = context.get(value) {
        return Some(v.clone());
    }
    Some(parse_recipe_value(value))
}

fn apply_extract_transforms(field: &str, value: &str, spec: &KotobaParserSpec) -> String {
    let mut out = value.to_string();
    for transform in &spec.transforms {
        if transform.on_extract
            && (transform.field == field
                || transform.field == "*"
                || transform.field == "text" && field == "text")
        {
            out = out.replace(&transform.from, &transform.to);
        }
    }
    out
}

fn apply_rebuild_transforms(field: &str, value: &str, spec: &KotobaParserSpec) -> String {
    let mut out = value.to_string();
    for transform in &spec.transforms {
        if transform.on_rebuild
            && (transform.field == field
                || transform.field == "*"
                || transform.field == "text" && field == "text")
        {
            out = out.replace(&transform.from, &transform.to);
        }
    }
    out
}

fn protected_token_warning(
    spec: &KotobaParserSpec,
    source: &str,
    translation: &str,
    entry_id: &str,
) -> Option<String> {
    let source_tokens = protected_tokens(spec, source);
    if source_tokens.is_empty() {
        return None;
    }
    let translation_tokens = protected_tokens(spec, translation);
    let mut missing = Vec::new();
    for token in source_tokens {
        if !translation_tokens.contains(&token) {
            missing.push(token);
        }
    }
    if missing.is_empty() {
        None
    } else {
        Some(format!(
            "entry {}: tradução não preserva tags protegidas: {}",
            entry_id,
            missing.join(", ")
        ))
    }
}

fn protected_tokens(spec: &KotobaParserSpec, text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for rule in &spec.protect {
        match rule {
            KotobaProtectRule::Literal(value) => {
                if value.is_empty() {
                    continue;
                }
                let mut rest = text;
                while let Some(pos) = rest.find(value) {
                    out.push(value.clone());
                    rest = &rest[pos + value.len()..];
                }
            }
            KotobaProtectRule::Pattern(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    for m in re.find_iter(text) {
                        out.push(m.as_str().to_string());
                    }
                }
            }
        }
    }
    out.sort();
    out
}

fn find_record_block_end(lines: &[&str], start: usize, block: &KotobaBlockSpec) -> Option<usize> {
    if block.end == "__balanced_braces__" {
        let mut depth: isize = 0;
        let mut seen_start = false;
        for i in start..lines.len() {
            let line = lines[i];
            if !seen_start {
                if !line.contains(&block.start) {
                    continue;
                }
                seen_start = true;
            }
            depth += brace_delta_outside_strings(line);
            if i > start && depth <= 0 {
                return Some(i);
            }
        }
        return None;
    }
    let mut end = start;
    while end < lines.len() && !lines[end].contains(&block.end) {
        end += 1;
    }
    if end < lines.len() {
        Some(end)
    } else {
        None
    }
}

fn brace_delta_outside_strings(line: &str) -> isize {
    let mut delta = 0isize;
    let mut in_string: Option<char> = None;
    let mut escape = false;
    for ch in line.chars() {
        if let Some(quote) = in_string {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == quote {
                in_string = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            in_string = Some(ch);
        } else if ch == '{' {
            delta += 1;
        } else if ch == '}' {
            delta -= 1;
        }
    }
    delta
}

fn extract_block_entries(
    source: &str,
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let Some(block) = spec
            .blocks
            .iter()
            .find(|block| !block.start.is_empty() && lines[i].contains(&block.start))
        else {
            i += 1;
            continue;
        };
        let start = i;
        let Some(end) = find_record_block_end(&lines, start, block) else {
            warnings.push(format!(
                "block {} iniciado na linha {} sem delimitador final",
                block.name,
                start + 1
            ));
            break;
        };
        let joined = lines[start..=end].join("\n");
        let rule_names: BTreeSet<String> = block.rules.iter().cloned().collect();
        let mut context = initial_context(spec);
        for rule in &spec.rules {
            if !rule_names.is_empty() && !rule_names.contains(&rule.name) {
                continue;
            }
            let matched = match match_rule_across_block_lines(rule, &joined, spec)? {
                Some(matched) => matched,
                None => match match_rule_with_spans_at(rule, &joined, spec, 0)? {
                    Some(matched) => matched,
                    None => continue,
                },
            };
            if !rule_conditions_match(&rule.when, &matched.captures, &context) {
                continue;
            }
            for key in &rule.forget {
                context.remove(key);
            }
            for directive in &rule.set {
                if let Some(value) =
                    resolve_set_value(&directive.value, &matched.captures, &context, spec)
                {
                    context.insert(directive.name.clone(), value);
                }
            }
            remember_rule_captures(rule, &matched.captures, &mut context, spec);
            if rule.skip || rule.entry_type.is_none() {
                continue;
            }
            let entry_type = rule
                .entry_type
                .clone()
                .unwrap_or_else(|| "Narration".into());
            let text_field = rule.text_field.clone().unwrap_or_else(|| "text".into());
            let Some(raw_text) = matched
                .captures
                .get(&text_field)
                .or_else(|| context.get(&text_field))
            else {
                warnings.push(format!(
                    "block {} linha {}: regra {} não capturou {}",
                    block.name,
                    start + 1,
                    rule.name,
                    text_field
                ));
                continue;
            };
            let source_type = capture_type_for(rule, &text_field).unwrap_or("line");
            let text = apply_extract_transforms(
                &text_field,
                &normalize_captured(raw_text, source_type, spec),
                spec,
            );
            if text.trim().is_empty() {
                continue;
            }
            let speaker = rule.speaker_field.as_ref().and_then(|field| {
                resolve_runtime_field(field, &matched.captures, &context, rule, spec)
            });
            let context_value = rule.context_field.as_ref().and_then(|field| {
                resolve_runtime_field(field, &matched.captures, &context, rule, spec)
            });
            let mut fields = normalized_fields(&matched.captures, spec);
            enrich_derived_fields(&mut fields);
            for (k, v) in &context {
                fields.entry(k.clone()).or_insert_with(|| v.clone());
            }
            fields.insert("block".into(), block.name.clone());
            fields.insert("block_start".into(), (start + 1).to_string());
            fields.insert("block_end".into(), (end + 1).to_string());
            fields.insert("selected".into(), text.clone());
            if let Some(s) = &speaker {
                fields.insert("selected_speaker".into(), s.clone());
            }
            entries.push(KotobaEntry {
                id: format!(
                    "{}_b{:05}_{}",
                    sanitize_id(&rule.name),
                    start + 1,
                    entries.len() + 1
                ),
                index: entries.len(),
                kind: kind_to_snake(&entry_type),
                speaker,
                text,
                context: context_value,
                line: start + 1,
                rule: rule.name.clone(),
                fields,
                patch_field: rule.patch_field.clone().unwrap_or(text_field),
                speaker_patch_field: rule.speaker_patch_field.clone(),
            });
        }
        i = end + 1;
    }
    Ok((
        entries.clone(),
        KotobaExtractReport {
            total_entries: entries.len(),
            warnings,
        },
    ))
}

fn apply_block_entry_field_patch(
    lines: &mut Vec<String>,
    spec: &KotobaParserSpec,
    entry: &KotobaEntry,
    field: &str,
    replacement: &str,
) -> Result<bool, KotobaError> {
    let start = entry
        .fields
        .get("block_start")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(entry.line);
    let end = entry
        .fields
        .get("block_end")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(entry.line);
    if start == 0 || end < start || end > lines.len() {
        return Ok(false);
    }
    let Some(rule) = spec.rules.iter().find(|rule| rule.name == entry.rule) else {
        return Ok(false);
    };

    // `lines` comes from split_preserving_newline(), so each physical line may
    // already include its trailing newline. Joining those raw strings with an
    // extra "\n" creates blank lines inside the block and breaks multiline
    // rule matching. Rebuild block text from logical line bodies instead, then
    // add the original final newline back after patching.
    let original_slice = &lines[start - 1..end];
    let had_trailing_newline = original_slice
        .last()
        .map(|line| line.ends_with('\n') || line.ends_with('\r'))
        .unwrap_or(false);
    let final_line_ending = original_slice
        .last()
        .and_then(|line| {
            if line.ends_with("\r\n") {
                Some("\r\n")
            } else if line.ends_with('\n') {
                Some("\n")
            } else if line.ends_with('\r') {
                Some("\r")
            } else {
                None
            }
        })
        .unwrap_or("\n");
    let mut block_text = original_slice
        .iter()
        .map(|line| line_body(line).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let matched = match match_rule_across_block_lines(rule, &block_text, spec)? {
        Some(matched) => matched,
        None => match match_rule_with_spans_at(rule, &block_text, spec, 0)? {
            Some(matched) => matched,
            None => return Ok(false),
        },
    };
    let Some((span_start, span_end)) = matched.spans.get(field).copied() else {
        return Ok(false);
    };
    let raw = &block_text[span_start..span_end];
    let replacement = apply_rebuild_transforms(field, replacement, spec);
    let capture_type = capture_type_for(rule, field).unwrap_or("line");
    let patched = format_replacement_for_capture(raw, &replacement, capture_type, spec);
    block_text.replace_range(span_start..span_end, &patched);
    if had_trailing_newline {
        block_text.push_str(final_line_ending);
    }
    let replacement_lines = split_preserving_newline(&block_text);
    lines.splice(start - 1..end, replacement_lines);
    Ok(true)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum JsonPathToken {
    Key(String),
    Wildcard,
}

fn extract_structured_json(
    source: &str,
    spec: &KotobaParserSpec,
) -> Result<(Vec<KotobaEntry>, KotobaExtractReport), KotobaError> {
    let value: serde_json::Value =
        serde_json::from_str(source).map_err(|e| KotobaError::Parse {
            line: e.line(),
            message: format!("JSON inválido: {}", e),
        })?;
    let nodes = collect_json_entry_nodes(&value, spec);
    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    for node in nodes {
        let text = get_json_string_at(
            &value,
            &node.node_path,
            &parse_relative_json_path(&node.spec.text),
        );
        let Some(text) = text else {
            warnings.push(format!(
                "json {} em {} não possui campo de texto {}",
                node.spec.name,
                json_path_label(&node.node_path),
                node.spec.text
            ));
            continue;
        };
        let text = apply_extract_transforms("text", &text, spec);
        if text.trim().is_empty() {
            continue;
        }
        let speaker = node
            .spec
            .speaker
            .as_ref()
            .and_then(|path| {
                get_json_string_at(&value, &node.node_path, &parse_relative_json_path(path))
            })
            .map(|v| apply_extract_transforms("speaker", &v, spec));
        let context = node.spec.context.as_ref().and_then(|path| {
            get_json_string_at(&value, &node.node_path, &parse_relative_json_path(path))
        });
        let id_value = node.spec.id.as_ref().and_then(|path| {
            get_json_string_at(&value, &node.node_path, &parse_relative_json_path(path))
        });
        let mut fields = BTreeMap::new();
        fields.insert("json_path".into(), json_path_label(&node.node_path));
        fields.insert("selected".into(), text.clone());
        if let Some(s) = &speaker {
            fields.insert("selected_speaker".into(), s.clone());
        }
        entries.push(KotobaEntry {
            id: id_value.unwrap_or_else(|| format!("json_{:05}", entries.len() + 1)),
            index: entries.len(),
            kind: if speaker.is_some() {
                "dialogue".into()
            } else {
                "narration".into()
            },
            speaker,
            text,
            context,
            line: find_json_line_for_text(
                source,
                &fields.get("selected").cloned().unwrap_or_default(),
            ),
            rule: node.spec.name.clone(),
            fields,
            patch_field: "text".into(),
            speaker_patch_field: node.spec.speaker.clone(),
        });
    }
    Ok((
        entries.clone(),
        KotobaExtractReport {
            total_entries: entries.len(),
            warnings,
        },
    ))
}

#[derive(Debug, Clone)]
struct JsonEntryNode<'a> {
    spec: &'a KotobaJsonPathSpec,
    node_path: Vec<JsonPathToken>,
}

fn collect_json_entry_nodes<'a>(
    value: &serde_json::Value,
    spec: &'a KotobaParserSpec,
) -> Vec<JsonEntryNode<'a>> {
    let mut out = Vec::new();
    for json_spec in &spec.json_paths {
        let tokens = parse_json_path(&json_spec.entries);
        let mut paths = Vec::new();
        collect_json_paths(value, &tokens, Vec::new(), &mut paths);
        for node_path in paths {
            out.push(JsonEntryNode {
                spec: json_spec,
                node_path,
            });
        }
    }
    out
}

fn rebuild_structured_json(
    source: &str,
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<(String, KotobaRebuildReport), KotobaError> {
    let mut value: serde_json::Value =
        serde_json::from_str(source).map_err(|e| KotobaError::Parse {
            line: e.line(),
            message: format!("JSON inválido: {}", e),
        })?;
    let (entries, _) = extract_structured_json(source, spec)?;
    let nodes = collect_json_entry_nodes(&value, spec);
    let mut report = KotobaRebuildReport {
        total_patches: patches.len(),
        ..KotobaRebuildReport::default()
    };
    let mut changed_lines = BTreeSet::new();
    for (pos, patch) in patches.iter().enumerate() {
        if patch.translation.trim().is_empty() {
            report.skipped_patches += 1;
            continue;
        }
        let entry = if !patch.id.trim().is_empty() {
            entries.iter().find(|e| e.id == patch.id)
        } else {
            entries.get(patch.index).or_else(|| entries.get(pos))
        };
        let Some(entry) = entry else {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: entry JSON não encontrada",
                patch_label(pos, patch)
            ));
            continue;
        };
        if !patch.source.trim().is_empty() && patch.source != entry.text {
            report.skipped_patches += 1;
            report.warnings.push(format!(
                "patch {} ignorado: source divergente",
                patch_label(pos, patch)
            ));
            continue;
        }
        let Some(node) = nodes.get(entry.index) else {
            report.skipped_patches += 1;
            continue;
        };
        if let Some(warning) =
            protected_token_warning(spec, &entry.text, &patch.translation, &entry.id)
        {
            report.warnings.push(warning);
        }
        let text_path = parse_relative_json_path(&node.spec.text);
        let replacement = apply_rebuild_transforms("text", &patch.translation, spec);
        if set_json_string_at(&mut value, &node.node_path, &text_path, replacement) {
            report.applied_patches += 1;
            if entry.line > 0 {
                changed_lines.insert(entry.line);
            }
        } else {
            report.skipped_patches += 1;
        }
    }
    report.changed_lines = changed_lines.into_iter().collect();
    let rebuilt = serde_json::to_string_pretty(&value).map_err(|e| KotobaError::Parse {
        line: 1,
        message: e.to_string(),
    })?;
    Ok((rebuilt, report))
}

fn preview_structured_json(
    source: &str,
    spec: &KotobaParserSpec,
    patches: &[KotobaPatchInput],
) -> Result<KotobaRebuildPreview, KotobaError> {
    let (entries, _) = extract_structured_json(source, spec)?;
    let (rebuilt, report) = rebuild_structured_json(source, spec, patches)?;
    let mut changes = Vec::new();
    if report.applied_patches > 0 {
        changes.push(KotobaLineChange {
            line: 0,
            before: source.to_string(),
            after: rebuilt,
            entries: entries
                .into_iter()
                .map(|entry| changed_entry_from_entry(&entry))
                .collect(),
        });
    }
    Ok(KotobaRebuildPreview {
        changed: report.applied_patches > 0,
        report,
        changes,
    })
}

fn parse_json_path(path: &str) -> Vec<JsonPathToken> {
    let mut out = Vec::new();
    let mut rest = path.trim().trim_start_matches('$');
    if rest.starts_with('.') {
        rest = &rest[1..];
    }
    for part in rest.split('.') {
        if part.is_empty() {
            continue;
        }
        if let Some(key) = part.strip_suffix("[*]") {
            if !key.is_empty() {
                out.push(JsonPathToken::Key(key.to_string()));
            }
            out.push(JsonPathToken::Wildcard);
        } else if part == "[*]" || part == "*" {
            out.push(JsonPathToken::Wildcard);
        } else {
            out.push(JsonPathToken::Key(part.trim_matches('"').to_string()));
        }
    }
    out
}

fn parse_relative_json_path(path: &str) -> Vec<JsonPathToken> {
    parse_json_path(path.trim().trim_start_matches('.'))
}

fn collect_json_paths(
    value: &serde_json::Value,
    tokens: &[JsonPathToken],
    path: Vec<JsonPathToken>,
    out: &mut Vec<Vec<JsonPathToken>>,
) {
    if tokens.is_empty() {
        out.push(path);
        return;
    }
    match &tokens[0] {
        JsonPathToken::Key(key) => {
            if let Some(next) = value.get(key.as_str()) {
                let mut next_path = path;
                next_path.push(JsonPathToken::Key(key.clone()));
                collect_json_paths(next, &tokens[1..], next_path, out);
            }
        }
        JsonPathToken::Wildcard => {
            if let Some(array) = value.as_array() {
                for (index, item) in array.iter().enumerate() {
                    let mut next_path = path.clone();
                    next_path.push(JsonPathToken::Key(index.to_string()));
                    collect_json_paths(item, &tokens[1..], next_path, out);
                }
            }
        }
    }
}

fn get_json_string_at(
    value: &serde_json::Value,
    base: &[JsonPathToken],
    relative: &[JsonPathToken],
) -> Option<String> {
    let target = get_json_value_at(value, base)?.clone();
    let target = get_json_value_at(&target, relative)?;
    match target {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn get_json_value_at<'a>(
    value: &'a serde_json::Value,
    path: &[JsonPathToken],
) -> Option<&'a serde_json::Value> {
    let mut current = value;
    for token in path {
        match token {
            JsonPathToken::Key(key) => {
                if let Ok(index) = key.parse::<usize>() {
                    current = current.as_array()?.get(index)?;
                } else {
                    current = current.get(key.as_str())?;
                }
            }
            JsonPathToken::Wildcard => return None,
        }
    }
    Some(current)
}

fn set_json_string_at(
    value: &mut serde_json::Value,
    base: &[JsonPathToken],
    relative: &[JsonPathToken],
    replacement: String,
) -> bool {
    let mut full = base.to_vec();
    full.extend_from_slice(relative);
    let Some((last, parents)) = full.split_last() else {
        return false;
    };
    let mut current = value;
    for token in parents {
        match token {
            JsonPathToken::Key(key) => {
                if let Ok(index) = key.parse::<usize>() {
                    let Some(array) = current.as_array_mut() else {
                        return false;
                    };
                    let Some(next) = array.get_mut(index) else {
                        return false;
                    };
                    current = next;
                } else {
                    let Some(next) = current.get_mut(key.as_str()) else {
                        return false;
                    };
                    current = next;
                }
            }
            JsonPathToken::Wildcard => return false,
        }
    }
    match last {
        JsonPathToken::Key(key) => {
            if let Ok(index) = key.parse::<usize>() {
                let Some(array) = current.as_array_mut() else {
                    return false;
                };
                if let Some(slot) = array.get_mut(index) {
                    *slot = serde_json::Value::String(replacement);
                    return true;
                }
            } else if let Some(object) = current.as_object_mut() {
                object.insert(key.clone(), serde_json::Value::String(replacement));
                return true;
            }
        }
        JsonPathToken::Wildcard => return false,
    }
    false
}

fn json_path_label(path: &[JsonPathToken]) -> String {
    let mut out = String::from("$");
    for token in path {
        match token {
            JsonPathToken::Key(key) if key.parse::<usize>().is_ok() => {
                out.push_str(&format!("[{}]", key))
            }
            JsonPathToken::Key(key) => out.push_str(&format!(".{}", key)),
            JsonPathToken::Wildcard => out.push_str("[*]"),
        }
    }
    out
}

fn find_json_line_for_text(source: &str, text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    if let Some(pos) = source.find(text) {
        source[..pos].lines().count() + 1
    } else {
        0
    }
}

fn changed_entry_from_entry(entry: &KotobaEntry) -> KotobaChangedEntry {
    KotobaChangedEntry {
        id: entry.id.clone(),
        index: entry.index,
        kind: entry.kind.clone(),
        rule: entry.rule.clone(),
        speaker: entry.speaker.clone(),
        text: entry.text.clone(),
    }
}

fn patch_label(pos: usize, patch: &KotobaPatchInput) -> String {
    if patch.id.trim().is_empty() {
        format!("#{}", pos)
    } else {
        patch.id.clone()
    }
}

fn apply_entry_field_patch(
    line: &mut String,
    spec: &KotobaParserSpec,
    entry: &KotobaEntry,
    field: &str,
    replacement: &str,
) -> Result<bool, KotobaError> {
    let Some(rule) = spec.rules.iter().find(|rule| rule.name == entry.rule) else {
        return Ok(false);
    };
    let Some(matched) = match_rule_with_spans(rule, line_body(line), spec)? else {
        return Ok(false);
    };
    let Some((start, end)) = matched.spans.get(field).copied() else {
        return Ok(false);
    };
    let raw = &line[start..end];

    if entry.kind == "choice" {
        if let Some(choice_key) = entry
            .fields
            .get("choice_key")
            .or_else(|| entry.context.as_ref())
        {
            let patched_cell = patch_choice_cell(raw, choice_key, &entry.text, replacement);
            if patched_cell != raw {
                line.replace_range(start..end, &patched_cell);
                return Ok(true);
            }
        }
    }

    let replacement = apply_rebuild_transforms(field, replacement, spec);
    let capture_type = capture_type_for(rule, field).unwrap_or("line");
    let patched = format_replacement_for_capture(raw, &replacement, capture_type, spec);
    line.replace_range(start..end, &patched);
    Ok(true)
}

fn plan_line_entry_field_edit(
    line: &str,
    line_offset: usize,
    spec: &KotobaParserSpec,
    entry: &KotobaEntry,
    field: &str,
    replacement: &str,
    patch_id: &str,
) -> Result<Option<KotobaTextEdit>, KotobaError> {
    let Some(rule) = spec.rules.iter().find(|rule| rule.name == entry.rule) else {
        return Ok(None);
    };
    let Some(matched) = match_rule_with_spans(rule, line_body(line), spec)? else {
        return Ok(None);
    };
    let Some((capture_start, capture_end)) = matched.spans.get(field).copied() else {
        return Ok(None);
    };
    let raw = &line[capture_start..capture_end];
    let replacement = apply_rebuild_transforms(field, replacement, spec);
    let capture_type = capture_type_for(rule, field).unwrap_or("line");
    let Some((relative_start, relative_end, replacement)) =
        replacement_edit_for_capture(raw, &replacement, capture_type, spec, entry)
    else {
        return Ok(None);
    };
    Ok(Some(KotobaTextEdit {
        start: line_offset + capture_start + relative_start,
        end: line_offset + capture_start + relative_end,
        replacement,
        patch_id: patch_id.to_string(),
        entry_id: entry.id.clone(),
        entry_index: entry.index,
        field: field.to_string(),
    }))
}

fn plan_block_entry_field_edit(
    lines: &[String],
    line_offsets: &[usize],
    spec: &KotobaParserSpec,
    entry: &KotobaEntry,
    field: &str,
    replacement: &str,
    patch_id: &str,
) -> Result<Option<KotobaTextEdit>, KotobaError> {
    let start = entry
        .fields
        .get("block_start")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(entry.line);
    let end = entry
        .fields
        .get("block_end")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(entry.line);
    if start == 0 || end < start || end > lines.len() {
        return Ok(None);
    }
    let Some(rule) = spec.rules.iter().find(|rule| rule.name == entry.rule) else {
        return Ok(None);
    };
    let block_lines = &lines[start - 1..end];
    let block_text = block_lines
        .iter()
        .map(|line| line_body(line))
        .collect::<Vec<_>>()
        .join("\n");
    let matched = match match_rule_across_block_lines(rule, &block_text, spec)? {
        Some(matched) => matched,
        None => match match_rule_with_spans_at(rule, &block_text, spec, 0)? {
            Some(matched) => matched,
            None => return Ok(None),
        },
    };
    let Some((capture_start, capture_end)) = matched.spans.get(field).copied() else {
        return Ok(None);
    };
    let raw = &block_text[capture_start..capture_end];
    let replacement = apply_rebuild_transforms(field, replacement, spec);
    let capture_type = capture_type_for(rule, field).unwrap_or("line");
    let Some((relative_start, relative_end, replacement)) =
        replacement_edit_for_capture(raw, &replacement, capture_type, spec, entry)
    else {
        return Ok(None);
    };
    let logical_start = capture_start + relative_start;
    let logical_end = capture_start + relative_end;
    let Some(physical_start) = block_logical_offset_to_source_offset(
        block_lines,
        &line_offsets[start - 1..end],
        logical_start,
    ) else {
        return Ok(None);
    };
    let Some(physical_end) = block_logical_offset_to_source_offset(
        block_lines,
        &line_offsets[start - 1..end],
        logical_end,
    ) else {
        return Ok(None);
    };
    Ok(Some(KotobaTextEdit {
        start: physical_start,
        end: physical_end,
        replacement,
        patch_id: patch_id.to_string(),
        entry_id: entry.id.clone(),
        entry_index: entry.index,
        field: field.to_string(),
    }))
}

fn block_logical_offset_to_source_offset(
    lines: &[String],
    line_offsets: &[usize],
    logical_offset: usize,
) -> Option<usize> {
    let mut logical_cursor = 0usize;
    for (index, line) in lines.iter().enumerate() {
        let body_len = line_body(line).len();
        if logical_offset <= logical_cursor + body_len {
            return Some(line_offsets[index] + logical_offset - logical_cursor);
        }
        logical_cursor += body_len;
        if index + 1 < lines.len() {
            if logical_offset == logical_cursor {
                return Some(line_offsets[index] + body_len);
            }
            logical_cursor += 1;
        }
    }
    (logical_offset == logical_cursor).then(|| {
        line_offsets.last().copied().unwrap_or(0)
            + lines.last().map(|line| line_body(line).len()).unwrap_or(0)
    })
}

fn replacement_edit_for_capture(
    raw_capture: &str,
    replacement: &str,
    capture_type: &str,
    spec: &KotobaParserSpec,
    entry: &KotobaEntry,
) -> Option<(usize, usize, String)> {
    if entry.kind == "choice" {
        if let Some(choice_key) = entry
            .fields
            .get("choice_key")
            .or_else(|| entry.context.as_ref())
        {
            if let Some(edit) =
                choice_replacement_edit(raw_capture, choice_key, &entry.text, replacement)
            {
                return Some(edit);
            }
        }
    }

    let leading_len = raw_capture.len() - raw_capture.trim_start().len();
    let core_end = raw_capture.trim_end().len();
    if leading_len > core_end {
        return Some((0, raw_capture.len(), replacement.to_string()));
    }
    let core = &raw_capture[leading_len..core_end];
    if capture_type == "quoted" || strip_quote_pair(core, spec).is_some() {
        if let Some((open, close)) = detect_quote_pair(core, spec) {
            let content_start = leading_len + open.len();
            let content_end = core_end.saturating_sub(close.len());
            return Some((
                content_start,
                content_end,
                escape_for_quote_pair(replacement, &open, &close),
            ));
        }
    }
    Some((leading_len, core_end, replacement.to_string()))
}

fn choice_replacement_edit(
    raw_cell: &str,
    choice_key: &str,
    source_text: &str,
    replacement: &str,
) -> Option<(usize, usize, String)> {
    let pattern = format!(
        r#"(?P<prefix>(?:^|\s){}\s*:\s*")(?P<text>(?:\\.|[^"])*)""#,
        regex::escape(choice_key)
    );
    let re = Regex::new(&pattern).ok()?;
    let caps = re.captures(raw_cell)?;
    let text_match = caps.name("text")?;
    let current_text = text_match.as_str().replace("\\\"", "\"");
    if current_text != source_text {
        return None;
    }
    Some((
        text_match.start(),
        text_match.end(),
        escape_for_quote_pair(replacement, "\"", "\""),
    ))
}

#[derive(Debug, Clone)]
struct KotobaRuleMatch {
    captures: BTreeMap<String, String>,
    spans: BTreeMap<String, (usize, usize)>,
}

fn match_rule_with_spans(
    rule: &KotobaRule,
    line: &str,
    spec: &KotobaParserSpec,
) -> Result<Option<KotobaRuleMatch>, KotobaError> {
    let (match_line, base_offset) = if let Some(indexed) = &spec.line_indexed {
        if let Some((_idx, payload)) = strip_index_prefix(line, indexed) {
            (payload, line.len() - payload.len())
        } else {
            (line, 0)
        }
    } else {
        (line, 0)
    };

    if let Some(matched) = match_rule_with_spans_at(rule, match_line, spec, base_offset)? {
        return Ok(Some(matched));
    }
    let trimmed = match_line.trim();
    if trimmed != match_line {
        let offset = base_offset + leading_trim_byte_count(match_line);
        return match_rule_with_spans_at(rule, trimmed, spec, offset);
    }
    Ok(None)
}

fn match_rule_with_spans_at(
    rule: &KotobaRule,
    line: &str,
    spec: &KotobaParserSpec,
    offset: usize,
) -> Result<Option<KotobaRuleMatch>, KotobaError> {
    if rule.pattern.trim().is_empty() {
        return Ok(None);
    }
    if let Some(regex_src) = rule.pattern.trim().strip_prefix("regex:") {
        let re = Regex::new(&anchor_regex(regex_src)).map_err(|e| KotobaError::Regex {
            rule: rule.name.clone(),
            message: e.to_string(),
        })?;
        let Some(caps) = re.captures(line) else {
            return Ok(None);
        };
        return captures_from_direct_regex_captures(&re, caps, rule, spec, offset);
    }
    let (regex_src, capture_types) = compile_pattern(&rule.pattern);
    let re = Regex::new(&format!("^{}$", regex_src)).map_err(|e| KotobaError::Regex {
        rule: rule.name.clone(),
        message: e.to_string(),
    })?;
    let Some(caps) = re.captures(line) else {
        return Ok(None);
    };
    captures_from_regex_captures(caps, capture_types, spec, offset)
}

fn captures_from_direct_regex_captures(
    re: &Regex,
    caps: regex::Captures<'_>,
    rule: &KotobaRule,
    spec: &KotobaParserSpec,
    offset: usize,
) -> Result<Option<KotobaRuleMatch>, KotobaError> {
    let mut captures = BTreeMap::new();
    let mut spans = BTreeMap::new();
    for name in re.capture_names().flatten() {
        let Some(m) = caps.name(name) else {
            continue;
        };
        let value = m.as_str().to_string();
        if let Some(typ) = regex_capture_type(rule, name) {
            if !validate_capture(&value, typ, spec) {
                return Ok(None);
            }
        }
        captures.insert(name.to_string(), value);
        spans.insert(name.to_string(), (offset + m.start(), offset + m.end()));
    }
    Ok(Some(KotobaRuleMatch { captures, spans }))
}

fn captures_from_regex_captures(
    caps: regex::Captures<'_>,
    capture_types: BTreeMap<String, String>,
    spec: &KotobaParserSpec,
    offset: usize,
) -> Result<Option<KotobaRuleMatch>, KotobaError> {
    let mut captures = BTreeMap::new();
    let mut spans = BTreeMap::new();
    for (field, typ) in capture_types {
        let Some(m) = caps.name(&field) else {
            continue;
        };
        let value = m.as_str().to_string();
        if !validate_capture(&value, &typ, spec) {
            return Ok(None);
        }
        captures.insert(field.clone(), value);
        spans.insert(field, (offset + m.start(), offset + m.end()));
    }
    Ok(Some(KotobaRuleMatch { captures, spans }))
}

fn match_rule_across_block_lines(
    rule: &KotobaRule,
    joined: &str,
    spec: &KotobaParserSpec,
) -> Result<Option<KotobaRuleMatch>, KotobaError> {
    let normalized_pattern = rule.pattern.replace("\\n", "\n");
    if !normalized_pattern.contains('\n') {
        return Ok(None);
    }
    let pattern_lines: Vec<&str> = normalized_pattern.lines().collect();
    let source_lines: Vec<&str> = joined.lines().collect();
    if pattern_lines.len() != source_lines.len() {
        return Ok(None);
    }

    let mut captures = BTreeMap::new();
    let mut spans = BTreeMap::new();
    let mut offset = 0usize;
    for (pattern, source) in pattern_lines.iter().zip(source_lines.iter()) {
        let segment_rule = KotobaRule {
            pattern: (*pattern).to_string(),
            ..rule.clone()
        };
        let matched = match match_rule_with_spans_at(&segment_rule, source, spec, offset)? {
            Some(matched) => matched,
            None => {
                let trimmed = source.trim();
                if trimmed == *source {
                    return Ok(None);
                }
                let trimmed_offset = offset + leading_trim_byte_count(source);
                let Some(matched) =
                    match_rule_with_spans_at(&segment_rule, trimmed, spec, trimmed_offset)?
                else {
                    return Ok(None);
                };
                matched
            }
        };
        for (key, value) in matched.captures {
            captures.insert(key, value);
        }
        for (key, span) in matched.spans {
            spans.insert(key, span);
        }
        offset += source.len() + 1;
    }
    Ok(Some(KotobaRuleMatch { captures, spans }))
}

fn line_body(line: &str) -> &str {
    line.strip_suffix("\r\n")
        .or_else(|| line.strip_suffix('\n'))
        .or_else(|| line.strip_suffix('\r'))
        .unwrap_or(line)
}

fn leading_trim_byte_count(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

fn format_replacement_for_capture(
    raw_capture: &str,
    replacement: &str,
    capture_type: &str,
    spec: &KotobaParserSpec,
) -> String {
    let leading_len = raw_capture.len() - raw_capture.trim_start().len();
    let trailing_len = raw_capture.trim_end().len();
    if leading_len > trailing_len {
        return replacement.to_string();
    }
    let leading = &raw_capture[..leading_len];
    let core = &raw_capture[leading_len..trailing_len];
    let trailing = &raw_capture[trailing_len..];

    if capture_type == "quoted" || strip_quote_pair(core, spec).is_some() {
        if let Some((open, close)) = detect_quote_pair(core, spec) {
            let escaped = escape_for_quote_pair(replacement, &open, &close);
            return format!("{}{}{}{}{}", leading, open, escaped, close, trailing);
        }
    }

    format!("{}{}{}", leading, replacement, trailing)
}

fn detect_quote_pair(value: &str, spec: &KotobaParserSpec) -> Option<(String, String)> {
    let defaults = default_quote_pairs();
    for (open, close) in spec.quote_pairs.iter().chain(defaults.iter()) {
        if value.starts_with(open)
            && value.ends_with(close)
            && value.len() >= open.len() + close.len()
        {
            return Some((open.clone(), close.clone()));
        }
    }
    None
}

fn escape_for_quote_pair(value: &str, open: &str, _close: &str) -> String {
    if open == "\"" {
        value
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('"', "\\\"")
    } else {
        value.to_string()
    }
}

fn patch_choice_cell(
    raw_cell: &str,
    choice_key: &str,
    source_text: &str,
    replacement: &str,
) -> String {
    let pattern = format!(
        r#"(?P<prefix>(?:^|\s){}\s*:\s*")(?P<text>(?:\\.|[^"])*)""#,
        regex::escape(choice_key)
    );
    let Ok(re) = Regex::new(&pattern) else {
        return raw_cell.to_string();
    };
    let Some(caps) = re.captures(raw_cell) else {
        return raw_cell.to_string();
    };
    let Some(text_match) = caps.name("text") else {
        return raw_cell.to_string();
    };
    let current_text = text_match.as_str().replace("\\\"", "\"");
    if current_text != source_text {
        return raw_cell.to_string();
    }
    let mut out = raw_cell.to_string();
    out.replace_range(
        text_match.start()..text_match.end(),
        &escape_for_quote_pair(replacement, "\"", "\""),
    );
    out
}

fn apply_byte_preserving_edits(
    original_bytes: &[u8],
    source: &str,
    output_encoding: Option<&str>,
    edits: &[KotobaTextEdit],
    lossy: bool,
    substitutions: &[KotobaCharacterSubstitution],
    report: &mut KotobaRebuildReport,
) -> Result<Vec<u8>, KotobaError> {
    let source_encoding = detect_source_encoding(original_bytes);
    let output_encoding = output_encoding.unwrap_or(source_encoding).trim();
    let mut byte_edits = Vec::with_capacity(edits.len());
    for edit in edits {
        let start = source_offset_to_original_byte_offset(
            source,
            edit.start,
            source_encoding,
            original_bytes,
        )?;
        let end = source_offset_to_original_byte_offset(
            source,
            edit.end,
            source_encoding,
            original_bytes,
        )?;
        let replacement = encode_patch_replacement(
            &edit.replacement,
            output_encoding,
            edit,
            lossy,
            substitutions,
            report,
        )?;
        byte_edits.push((start, end, replacement));
    }
    byte_edits.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
    let mut rebuilt = original_bytes.to_vec();
    for (start, end, replacement) in byte_edits {
        if start > end || end > rebuilt.len() {
            return Err(KotobaError::Parse {
                line: 0,
                message: format!(
                    "intervalo de bytes inválido durante rebuild: {}..{} de {}",
                    start,
                    end,
                    rebuilt.len()
                ),
            });
        }
        rebuilt.splice(start..end, replacement);
    }
    Ok(rebuilt)
}

fn source_offset_to_original_byte_offset(
    source: &str,
    source_offset: usize,
    source_encoding: &str,
    original_bytes: &[u8],
) -> Result<usize, KotobaError> {
    if source_offset > source.len() || !source.is_char_boundary(source_offset) {
        return Err(KotobaError::Encoding {
            encoding: source_encoding.into(),
            message: format!(
                "offset textual inválido durante mapeamento: {}",
                source_offset
            ),
        });
    }
    let prefix = &source[..source_offset];
    let normalized = normalize_encoding_label(source_encoding);
    let bom_len = if original_bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        3
    } else if original_bytes.starts_with(&[0xFF, 0xFE]) || original_bytes.starts_with(&[0xFE, 0xFF])
    {
        2
    } else {
        0
    };
    let encoded_len = match normalized.as_str() {
        "utf8" | "utf-8" | "utf8-sig" | "utf-8-sig" => prefix.len(),
        "utf16" | "utf-16" | "utf16le" | "utf-16le" | "utf16be" | "utf-16be" => {
            prefix.encode_utf16().count() * 2
        }
        _ => {
            let Some(encoding) = encoding_for_label(&normalized) else {
                return Err(KotobaError::Encoding {
                    encoding: source_encoding.into(),
                    message: "encoding de origem não suportado para mapeamento de bytes".into(),
                });
            };
            let (encoded, _, had_errors) = encoding.encode(prefix);
            if had_errors {
                return Err(KotobaError::Encoding {
                    encoding: source_encoding.into(),
                    message: "não foi possível mapear o texto decodificado aos bytes originais"
                        .into(),
                });
            }
            encoded.len()
        }
    };
    let offset = bom_len + encoded_len;
    if offset > original_bytes.len() {
        return Err(KotobaError::Encoding {
            encoding: source_encoding.into(),
            message: format!(
                "mapeamento textual excedeu o arquivo original: {} > {}",
                offset,
                original_bytes.len()
            ),
        });
    }
    Ok(offset)
}

fn encode_patch_replacement(
    text: &str,
    encoding_name: &str,
    edit: &KotobaTextEdit,
    lossy: bool,
    substitutions: &[KotobaCharacterSubstitution],
    report: &mut KotobaRebuildReport,
) -> Result<Vec<u8>, KotobaError> {
    let normalized = normalize_encoding_label(encoding_name);
    match normalized.as_str() {
        "utf8" | "utf-8" | "utf8-sig" | "utf-8-sig" | "utf8-bom" | "utf-8-bom" => {
            if substitutions.is_empty() {
                return Ok(text.as_bytes().to_vec());
            }
        }
        "utf16" | "utf-16" | "utf16le" | "utf-16le" => {
            let mut out = Vec::new();
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_le_bytes());
            }
            if substitutions.is_empty() {
                return Ok(out);
            }
        }
        "utf16be" | "utf-16be" => {
            let mut out = Vec::new();
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_be_bytes());
            }
            if substitutions.is_empty() {
                return Ok(out);
            }
        }
        _ => {}
    }

    let encoding = encoding_for_label(&normalized);
    if encoding.is_none()
        && !matches!(
            normalized.as_str(),
            "utf8"
                | "utf-8"
                | "utf8-sig"
                | "utf-8-sig"
                | "utf8-bom"
                | "utf-8-bom"
                | "utf16"
                | "utf-16"
                | "utf16le"
                | "utf-16le"
                | "utf16be"
                | "utf-16be"
        )
    {
        return Err(KotobaError::Encoding {
            encoding: encoding_name.to_string(),
            message: "encoding não suportado pelo runtime Rust".into(),
        });
    }
    let mut out = Vec::new();
    let mut invalid = Vec::new();
    for (offset, character) in text.char_indices() {
        let character_text = character.to_string();
        let substitution = substitutions
            .iter()
            .find(|item| item.source == character_text);
        if let Some(substitution) = substitution {
            if substitution.mode.eq_ignore_ascii_case("bytes") {
                out.extend(
                    parse_substitution_hex(&substitution.target).map_err(|message| {
                        KotobaError::Encoding {
                            encoding: encoding_name.to_string(),
                            message: format!(
                                "substituição de {} na entry {} possui bytes inválidos: {}",
                                character, edit.entry_id, message
                            ),
                        }
                    })?,
                );
                continue;
            }
        }
        let encoded_text = substitution
            .map(|item| item.target.as_str())
            .unwrap_or(&character_text);
        let (encoded, had_errors) =
            encode_replacement_fragment(encoded_text, &normalized, encoding);
        if had_errors {
            invalid.push(KotobaUnencodableCharacter {
                character: character.to_string(),
                codepoint: format!("U+{:04X}", character as u32),
                replacement_offset: offset,
            });
            if lossy {
                out.push(b'?');
            }
        } else {
            out.extend_from_slice(&encoded);
        }
    }
    if invalid.is_empty() {
        return Ok(out);
    }
    let invalid_label = invalid
        .iter()
        .map(|item| format!("{} ({})", item.character, item.codepoint))
        .collect::<Vec<_>>()
        .join(", ");
    if !lossy {
        return Err(KotobaError::Encoding {
            encoding: encoding_name.to_string(),
            message: format!(
                "patch {} / entry {} / campo {} contém caractere(s) não representável(is): {}. \
Use rebuild --lossy somente se aceitar substituí-los por '?'.",
                edit.patch_id, edit.entry_id, edit.field, invalid_label
            ),
        });
    }
    report.lossy_replacements.push(KotobaLossyReplacement {
        patch_id: edit.patch_id.clone(),
        entry_id: edit.entry_id.clone(),
        entry_index: edit.entry_index,
        field: edit.field.clone(),
        encoding: encoding_name.to_string(),
        characters: invalid,
    });
    report.warnings.push(format!(
        "patch {} / entry {}: {} substituído(s) por '?' em {}",
        edit.patch_id, edit.entry_id, invalid_label, encoding_name
    ));
    Ok(out)
}

fn parse_substitution_hex(value: &str) -> Result<Vec<u8>, String> {
    let compact: String = value
        .chars()
        .filter(|character| {
            !character.is_ascii_whitespace() && !matches!(character, ',' | '_' | '-')
        })
        .collect();
    let compact = compact
        .strip_prefix("0x")
        .or_else(|| compact.strip_prefix("0X"))
        .unwrap_or(&compact);
    if compact.is_empty()
        || compact.len() % 2 != 0
        || !compact
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Err(format!(
            "\"{}\" não é uma sequência hexadecimal completa",
            value
        ));
    }
    (0..compact.len())
        .step_by(2)
        .map(|index| {
            u8::from_str_radix(&compact[index..index + 2], 16).map_err(|error| error.to_string())
        })
        .collect()
}

fn encode_replacement_fragment(
    text: &str,
    normalized_encoding: &str,
    encoding: Option<&'static encoding_rs::Encoding>,
) -> (Vec<u8>, bool) {
    match normalized_encoding {
        "utf8" | "utf-8" | "utf8-sig" | "utf-8-sig" | "utf8-bom" | "utf-8-bom" => {
            (text.as_bytes().to_vec(), false)
        }
        "utf16" | "utf-16" | "utf16le" | "utf-16le" => (
            text.encode_utf16()
                .flat_map(|unit| unit.to_le_bytes())
                .collect(),
            false,
        ),
        "utf16be" | "utf-16be" => (
            text.encode_utf16()
                .flat_map(|unit| unit.to_be_bytes())
                .collect(),
            false,
        ),
        _ => {
            let Some(encoding) = encoding else {
                return (Vec::new(), true);
            };
            let (encoded, _, had_errors) = encoding.encode(text);
            (encoded.into_owned(), had_errors)
        }
    }
}

fn encode_complete_rebuilt_text(
    text: &str,
    output_encoding: Option<&str>,
    original_bytes: &[u8],
    lossy: bool,
    report: &mut KotobaRebuildReport,
) -> Result<Vec<u8>, KotobaError> {
    let encoding_name = output_encoding
        .unwrap_or_else(|| detect_source_encoding(original_bytes))
        .trim();
    let normalized = normalize_encoding_label(encoding_name);

    if original_bytes.starts_with(&[0xEF, 0xBB, 0xBF])
        && matches!(
            normalized.as_str(),
            "utf8" | "utf-8" | "utf8-sig" | "utf-8-sig" | "utf8-bom" | "utf-8-bom"
        )
    {
        let mut out = vec![0xEF, 0xBB, 0xBF];
        out.extend_from_slice(text.as_bytes());
        return Ok(out);
    }

    if original_bytes.starts_with(&[0xFE, 0xFF])
        && matches!(
            normalized.as_str(),
            "utf16" | "utf-16" | "utf16be" | "utf-16be"
        )
    {
        let mut out = vec![0xFE, 0xFF];
        for unit in text.encode_utf16() {
            out.extend_from_slice(&unit.to_be_bytes());
        }
        return Ok(out);
    }

    if matches!(
        normalized.as_str(),
        "utf16" | "utf-16" | "utf16le" | "utf-16le"
    ) {
        let mut out = Vec::new();
        if original_bytes.starts_with(&[0xFF, 0xFE]) {
            out.extend_from_slice(&[0xFF, 0xFE]);
        }
        for unit in text.encode_utf16() {
            out.extend_from_slice(&unit.to_le_bytes());
        }
        return Ok(out);
    }

    if matches!(normalized.as_str(), "utf16be" | "utf-16be") {
        let mut out = Vec::new();
        if original_bytes.starts_with(&[0xFE, 0xFF]) {
            out.extend_from_slice(&[0xFE, 0xFF]);
        }
        for unit in text.encode_utf16() {
            out.extend_from_slice(&unit.to_be_bytes());
        }
        return Ok(out);
    }

    if !lossy {
        return encode_text(text, Some(encoding_name));
    }
    let synthetic_edit = KotobaTextEdit {
        start: 0,
        end: text.len(),
        replacement: text.to_string(),
        patch_id: "json".into(),
        entry_id: "json".into(),
        entry_index: 0,
        field: "document".into(),
    };
    encode_patch_replacement(text, encoding_name, &synthetic_edit, true, &[], report)
}

fn validate_spec(spec: &KotobaParserSpec) -> Vec<KotobaDiagnostic> {
    let mut diagnostics = Vec::new();
    if spec.name.trim().is_empty() {
        diagnostics.push(error_diag("parser sem name/parser", None, None));
    }
    if spec.id.trim().is_empty() {
        diagnostics.push(error_diag("parser sem id", None, None));
    }
    if spec.rules.is_empty() && spec.binary_blocks.is_empty() && spec.json_paths.is_empty() {
        diagnostics.push(warn_diag("parser não possui regras", None, None));
    }
    if let Some(encoding) = &spec.encoding {
        if spec.binary_blocks.is_empty()
            && encoding_for_label(&normalize_encoding_label(encoding)).is_none()
        {
            diagnostics.push(error_diag(
                format!("encoding não suportado: {}", encoding),
                None,
                None,
            ));
        }
    }

    for binary in &spec.binary_blocks {
        if binary.magic.is_empty() {
            diagnostics.push(error_diag(
                format!("binary block {} sem magic", binary.name),
                None,
                Some(binary.name.clone()),
            ));
        }
        if normalize_binary_length(&binary.length) != "u32le" {
            diagnostics.push(error_diag(
                format!(
                    "binary block {} usa length não suportado: {}",
                    binary.name, binary.length
                ),
                None,
                Some(binary.name.clone()),
            ));
        }
        if encoding_for_label(&normalize_encoding_label(&binary.encoding)).is_none() {
            diagnostics.push(error_diag(
                format!(
                    "binary block {} usa encoding não suportado: {}",
                    binary.name, binary.encoding
                ),
                None,
                Some(binary.name.clone()),
            ));
        }
    }

    for block in &spec.blocks {
        if block.start.is_empty() {
            diagnostics.push(error_diag(
                format!("block {} sem start", block.name),
                None,
                Some(block.name.clone()),
            ));
        }
        if block.end.is_empty() {
            diagnostics.push(error_diag(
                format!("block {} sem end", block.name),
                None,
                Some(block.name.clone()),
            ));
        }
        for rule_name in &block.rules {
            if !spec.rules.iter().any(|rule| &rule.name == rule_name) {
                diagnostics.push(warn_diag(
                    format!(
                        "block {} referencia rule desconhecida: {}",
                        block.name, rule_name
                    ),
                    None,
                    Some(block.name.clone()),
                ));
            }
        }
    }

    for json in &spec.json_paths {
        if json.entries.trim().is_empty() {
            diagnostics.push(error_diag(
                format!("json {} sem entries/path", json.name),
                None,
                Some(json.name.clone()),
            ));
        }
        if json.text.trim().is_empty() {
            diagnostics.push(error_diag(
                format!("json {} sem text", json.name),
                None,
                Some(json.name.clone()),
            ));
        }
    }

    for protect in &spec.protect {
        if let KotobaProtectRule::Pattern(pattern) = protect {
            if let Err(error) = Regex::new(pattern) {
                diagnostics.push(error_diag(
                    format!("protect pattern inválido: {}", error),
                    None,
                    None,
                ));
            }
        }
    }

    for (name, typ) in &spec.types {
        for pattern in &typ.patterns {
            if let Err(error) = Regex::new(pattern) {
                diagnostics.push(KotobaDiagnostic {
                    severity: "error".into(),
                    message: format!("regex inválido no type {}: {}", name, error),
                    line: None,
                    column: None,
                    rule: None,
                });
            }
        }
    }

    for rule in &spec.rules {
        if rule.pattern.trim().is_empty() {
            diagnostics.push(error_diag(
                "rule sem pattern",
                rule.source_line,
                Some(rule.name.clone()),
            ));
        } else {
            if let Some(regex_src) = rule.pattern.trim().strip_prefix("regex:") {
                if let Err(error) = Regex::new(&anchor_regex(regex_src)) {
                    diagnostics.push(KotobaDiagnostic {
                        severity: "error".into(),
                        message: format!("regex compilado inválido: {}", error),
                        line: rule.source_line,
                        column: None,
                        rule: Some(rule.name.clone()),
                    });
                }
                for (key, typ) in &rule.extra_fields {
                    if let Some(field) = key.strip_prefix("__type:") {
                        if !is_builtin_capture_type(typ) && !spec.types.contains_key(typ) {
                            diagnostics.push(KotobaDiagnostic {
                                severity: "error".into(),
                                message: format!("campo {} usa type desconhecido: {}", field, typ),
                                line: rule.source_line,
                                column: None,
                                rule: Some(rule.name.clone()),
                            });
                        }
                    }
                }
            } else {
                let (regex_src, capture_types) = compile_pattern(&rule.pattern);
                if let Err(error) = Regex::new(&format!("^{}$", regex_src)) {
                    diagnostics.push(KotobaDiagnostic {
                        severity: "error".into(),
                        message: format!("regex compilado inválido: {}", error),
                        line: rule.source_line,
                        column: None,
                        rule: Some(rule.name.clone()),
                    });
                }
                for (field, typ) in capture_types {
                    if !is_builtin_capture_type(&typ) && !spec.types.contains_key(&typ) {
                        diagnostics.push(KotobaDiagnostic {
                            severity: "error".into(),
                            message: format!("campo {} usa type desconhecido: {}", field, typ),
                            line: rule.source_line,
                            column: None,
                            rule: Some(rule.name.clone()),
                        });
                    }
                }
            }
        }

        if !rule.skip
            && rule.entry_type.is_none()
            && (!rule.remember.is_empty() || !rule.forget.is_empty())
        {
            // Context-only rules are valid.
        } else if !rule.skip && rule.entry_type.is_none() {
            diagnostics.push(warn_diag(
                "rule não gera entry e não é skip/remember/forget",
                rule.source_line,
                Some(rule.name.clone()),
            ));
        }

        if !rule.skip && rule.entry_type.is_some() && rule.text_field.is_none() {
            diagnostics.push(error_diag(
                "rule de entry sem campo de texto em `as ...(...)`",
                rule.source_line,
                Some(rule.name.clone()),
            ));
        }
    }
    diagnostics
}

fn diagnostic_from_error(error: KotobaError) -> KotobaDiagnostic {
    match error {
        KotobaError::Parse { line, message } => KotobaDiagnostic {
            severity: "error".into(),
            message,
            line: Some(line),
            column: None,
            rule: None,
        },
        KotobaError::Regex { rule, message } => KotobaDiagnostic {
            severity: "error".into(),
            message,
            line: None,
            column: None,
            rule: Some(rule),
        },
        KotobaError::Encoding { encoding, message } => KotobaDiagnostic {
            severity: "error".into(),
            message: format!("{}: {}", encoding, message),
            line: None,
            column: None,
            rule: None,
        },
    }
}

fn error_diag<M: Into<String>>(
    message: M,
    line: Option<usize>,
    rule: Option<String>,
) -> KotobaDiagnostic {
    KotobaDiagnostic {
        severity: "error".into(),
        message: message.into(),
        line,
        column: None,
        rule,
    }
}

fn warn_diag<M: Into<String>>(
    message: M,
    line: Option<usize>,
    rule: Option<String>,
) -> KotobaDiagnostic {
    KotobaDiagnostic {
        severity: "warning".into(),
        message: message.into(),
        line,
        column: None,
        rule,
    }
}

fn is_builtin_capture_type(typ: &str) -> bool {
    matches!(
        typ,
        "number" | "word" | "name" | "quoted" | "rest" | "line" | "cell"
    )
}

fn captures_from_pattern(pattern: &str) -> Vec<(String, String)> {
    if let Some(regex_src) = pattern.trim().strip_prefix("regex:") {
        if let Ok(re) = Regex::new(&anchor_regex(regex_src)) {
            return re
                .capture_names()
                .flatten()
                .map(|name| (name.to_string(), "regex".to_string()))
                .collect();
        }
    }
    let mut out = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(end) = chars[i + 1..].iter().position(|c| *c == '>') {
                let spec = chars[i + 1..i + 1 + end].iter().collect::<String>();
                if let Some((field, typ)) = spec.split_once(':') {
                    out.push((field.trim().to_string(), typ.trim().to_string()));
                }
                i += end + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn normalize_encoding_label(label: &str) -> String {
    label.trim().to_ascii_lowercase().replace('_', "-")
}

fn encoding_for_label(label: &str) -> Option<&'static Encoding> {
    match label {
        "utf8" | "utf-8" => Some(UTF_8),
        "utf8-sig" | "utf-8-sig" | "utf8-bom" | "utf-8-bom" => Some(UTF_8),
        "shift-jis" | "shift_jis" | "sjis" | "cp932" | "windows-31j" => Some(SHIFT_JIS),
        "windows-1252" | "cp1252" | "win1252" | "ansi" | "latin1" | "latin-1" => Some(WINDOWS_1252),
        "utf16" | "utf-16" | "utf16le" | "utf-16le" => Some(UTF_16LE),
        "utf16be" | "utf-16be" => Some(UTF_16BE),
        other => Encoding::for_label(other.as_bytes()),
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || start >= haystack.len() {
        return None;
    }
    haystack[start..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|pos| start + pos)
}

fn parse_hex_bytes(value: &str) -> Vec<u8> {
    let cleaned = value
        .replace("0x", "")
        .replace("\\x", "")
        .replace(',', " ")
        .replace('-', " ");
    cleaned
        .split_whitespace()
        .filter_map(|part| u8::from_str_radix(part, 16).ok())
        .collect()
}

fn normalize_binary_length(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('-', "")
}

fn decode_binary_payload(payload: &[u8], encoding: &str) -> Result<String, String> {
    let normalized = normalize_encoding_label(encoding);
    if normalized == "utf-8" || normalized == "utf8" {
        return String::from_utf8(payload.to_vec())
            .map_err(|e| format!("payload UTF-8 inválido: {}", e));
    }
    let Some(enc) = encoding_for_label(&normalized) else {
        return Err(format!("encoding não suportado: {}", encoding));
    };
    let (text, _, had_errors) = enc.decode(payload);
    if had_errors {
        return Err(format!("payload inválido para encoding {}", encoding));
    }
    Ok(text.into_owned())
}

fn normalize_binary_text_for_profile(
    raw: &str,
    profile: &str,
) -> (String, Option<String>, Option<String>) {
    let normalized_profile = profile.trim().to_ascii_lowercase().replace('-', "_");
    if normalized_profile == "gls_nut" || normalized_profile == "gls" || normalized_profile == "nut"
    {
        return normalize_gls_nut_text(raw);
    }
    (raw.trim().to_string(), None, None)
}

fn normalize_gls_nut_text(raw: &str) -> (String, Option<String>, Option<String>) {
    let speaker = capture_between(raw, "//【", "】");
    let voice = capture_between(raw, "src='", "'");
    let mut lines = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<voice ") {
            continue;
        }
        if trimmed.starts_with("//【") {
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        if trimmed.is_empty() {
            lines.push(String::new());
        } else {
            lines.push(trimmed.to_string());
        }
    }
    let joined = collapse_blank_edges(&lines.join("\n"));
    (joined, speaker, voice)
}

fn capture_between(source: &str, start: &str, end: &str) -> Option<String> {
    let s = source.find(start)? + start.len();
    let e = source[s..].find(end)? + s;
    let value = source[s..e].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn collapse_blank_edges(value: &str) -> String {
    let mut lines: Vec<&str> = value.lines().collect();
    while lines
        .first()
        .map(|line| line.trim().is_empty())
        .unwrap_or(false)
    {
        lines.remove(0);
    }
    while lines
        .last()
        .map(|line| line.trim().is_empty())
        .unwrap_or(false)
    {
        lines.pop();
    }
    lines.join("\n")
}

fn push_binary_string(
    bytes: &[u8],
    start: usize,
    end: usize,
    min_len: usize,
    out: &mut Vec<KotobaBinaryString>,
) {
    if end <= start || end - start < min_len {
        return;
    }
    let Ok(text) = std::str::from_utf8(&bytes[start..end]) else {
        return;
    };
    let cleaned = text
        .trim_matches(|c: char| c.is_control())
        .trim()
        .to_string();
    if cleaned.len() < min_len {
        return;
    }
    out.push(KotobaBinaryString {
        index: out.len(),
        offset: start,
        text: cleaned,
    });
}

fn looks_like_translatable_binary_text(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.len() < 4 {
        return false;
    }
    let lowered = trimmed.to_ascii_lowercase();
    if matches!(
        trimmed,
        "main" | "TransText" | "TransAddText" | "TransChoice" | "TransLog" | "TransVoice"
    ) {
        return false;
    }
    if lowered.starts_with("media/")
        || lowered.starts_with("voice/")
        || lowered.ends_with(".nut")
        || lowered.ends_with(".ogg")
        || lowered.ends_with(".png")
        || lowered.ends_with(".bmp")
    {
        return false;
    }
    if trimmed.starts_with("<voice ") && trimmed.ends_with('>') {
        return false;
    }
    if trimmed.starts_with("' class='") || trimmed.starts_with("' src='") {
        return false;
    }
    let has_letter = trimmed.chars().any(|c| c.is_alphabetic());
    let has_space_or_sentence = trimmed.chars().any(|c| c.is_whitespace())
        || trimmed.contains('.')
        || trimmed.contains('!')
        || trimmed.contains('?')
        || trimmed.contains('…')
        || trimmed.contains('。');
    has_letter && has_space_or_sentence
}

#[derive(Debug, Clone)]
struct RecipeLine {
    line_no: usize,
    indent: usize,
    text: String,
}

#[derive(Debug, Clone, Default)]
struct RecipeDialogueLineSpec {
    command: String,
    has_id: bool,
    has_voice: bool,
    voice_type: String,
    text_mode: String,
    plain_speaker_before_quoted: bool,
    marked_speaker_prefixes: Vec<String>,
    emit_narration_otherwise: bool,
    patch_field: String,
    speaker_patch_field: Option<String>,
    entry_type: String,
    narration_type: String,
    section: Option<String>,
    text_between: Option<(String, String)>,
    segment: Option<String>,
    speaker_between: Option<(String, String)>,
    text_after: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct RecipeBlockReadSpec {
    start: String,
    end: String,
    balanced_braces: bool,
}

#[derive(Debug, Clone, Default)]
struct RecipeSectionCaptureSpec {
    section: Option<String>,
    speaker_attr: Option<String>,
    voice_attr: Option<String>,
    remember_voice: bool,
    remember_speaker: bool,
    skip: bool,
}

#[derive(Debug, Clone, Default)]
struct RecipeSegmentsSpec {
    text_separator: Option<String>,
    choice_separator: Option<String>,
    fields: Vec<String>,
    source_field: Option<String>,
    patch_field: Option<String>,
}

impl RecipeSegmentsSpec {
    fn source_field(&self) -> String {
        self.source_field
            .clone()
            .or_else(|| self.fields.first().cloned())
            .unwrap_or_else(|| "text".into())
    }

    fn patch_field(&self) -> String {
        self.patch_field
            .clone()
            .unwrap_or_else(|| self.source_field())
    }

    fn has_text_segments(&self) -> bool {
        self.text_separator
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false)
            && !self.fields.is_empty()
    }

    fn has_choice_segments(&self) -> bool {
        self.choice_separator
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false)
            && !self.fields.is_empty()
    }
}

fn looks_like_kotoba_recipe(source: &str) -> bool {
    source
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .next()
        .map(|line| line.starts_with("parser ") && line.ends_with(':'))
        .unwrap_or(false)
}

fn parse_recipe_source(source: &str) -> Result<KotobaParserSpec, KotobaError> {
    let lines = recipe_lines(source);
    if lines.is_empty() {
        return Err(KotobaError::Parse {
            line: 1,
            message: "parser vazio".into(),
        });
    }
    let header = lines[0].text.trim();
    let Some(raw_name) = header
        .strip_prefix("parser ")
        .and_then(|v| v.strip_suffix(':'))
    else {
        return Err(KotobaError::Parse {
            line: lines[0].line_no,
            message: "esperado: parser Nome:".into(),
        });
    };
    validate_canonical_recipe(&lines)?;
    let name = parse_recipe_value(raw_name.trim());
    let mut spec = KotobaParserSpec {
        name: name.clone(),
        id: sanitize_id(&name),
        target: None,
        extensions: Vec::new(),
        encoding: None,
        line_indexed: None,
        types: BTreeMap::new(),
        skip_rules: Vec::new(),
        quote_pairs: default_quote_pairs(),
        protect: Vec::new(),
        binary_blocks: Vec::new(),
        states: Vec::new(),
        blocks: Vec::new(),
        json_paths: Vec::new(),
        transforms: Vec::new(),
        rebuild_strategy: None,
        rules: Vec::new(),
    };

    let mut i = 1usize;
    let mut recipe_segments = RecipeSegmentsSpec::default();
    while i < lines.len() {
        let line = &lines[i];
        let text = line.text.trim();
        if text.is_empty() {
            i += 1;
            continue;
        }

        if let Some(value) = text.strip_prefix("file ") {
            let target = parse_recipe_value(value);
            if !target.is_empty() {
                spec.target = Some(target.clone());
                if !spec.extensions.contains(&target) {
                    spec.extensions.push(target);
                }
            }
            i += 1;
            continue;
        }
        if let Some(value) = text.strip_prefix("encoding ") {
            spec.encoding = Some(parse_recipe_value(value));
            i += 1;
            continue;
        }
        if let Some(value) = text.strip_prefix("type ") {
            parse_recipe_type_inline(value, line.line_no, &mut spec)?;
            i += 1;
            continue;
        }

        if text.ends_with(':') {
            let block = text.trim_end_matches(':').trim();
            let (children, next) = recipe_child_lines(&lines, i + 1, line.indent);
            match block {
                "quotes" => parse_recipe_quotes(&children, &mut spec),
                "protect" => parse_recipe_protect(&children, &mut spec),
                "ignore" => {
                    parse_recipe_ignore(&children, &mut spec);
                }
                "voice" => parse_recipe_voice(&children, &mut spec),
                "speaker" => parse_recipe_speaker(&children, line.line_no, &mut spec),
                "dialogue" if recipe_uses_flexible_semantic_commands(&children) => {
                    parse_recipe_semantic_rule(
                        "dialogue",
                        None,
                        &children,
                        line.line_no,
                        &mut spec,
                    )?;
                }
                "dialogue" => {
                    parse_recipe_dialogue(&children, line.line_no, &mut spec, &recipe_segments)
                }
                "choice" if recipe_uses_flexible_semantic_commands(&children) => {
                    parse_recipe_semantic_rule("choice", None, &children, line.line_no, &mut spec)?;
                }
                "choice" => {
                    parse_recipe_choice(&children, line.line_no, &mut spec, &recipe_segments)
                }
                "read" => {
                    parse_recipe_read(&children, line.line_no, &mut spec, &mut recipe_segments)
                }
                block_name if block_name.starts_with("rule ") => parse_recipe_flexible_rule(
                    block_name.trim_start_matches("rule ").trim(),
                    &children,
                    line.line_no,
                    &mut spec,
                )?,
                block_name if block_name.starts_with("json ") => parse_recipe_json(
                    block_name.trim_start_matches("json ").trim(),
                    &children,
                    line.line_no,
                    &mut spec,
                )?,
                "text" if recipe_uses_flexible_semantic_commands(&children) => {
                    parse_recipe_semantic_rule("text", None, &children, line.line_no, &mut spec)?;
                }
                "text" => parse_recipe_text(&children, line.line_no, &mut spec, &recipe_segments),
                "numbered lines" => parse_recipe_numbered_lines(&children, &mut spec),
                block_name if recipe_named_semantic_block(block_name).is_some() => {
                    let (kind, name) = recipe_named_semantic_block(block_name)
                        .expect("named semantic block already checked");
                    if kind == "ignore" || recipe_uses_flexible_semantic_commands(&children) {
                        parse_recipe_semantic_rule(
                            kind,
                            Some(name),
                            &children,
                            line.line_no,
                            &mut spec,
                        )?;
                    } else {
                        match kind {
                            "dialogue" => parse_recipe_dialogue(
                                &children,
                                line.line_no,
                                &mut spec,
                                &recipe_segments,
                            ),
                            "text" => parse_recipe_text(
                                &children,
                                line.line_no,
                                &mut spec,
                                &recipe_segments,
                            ),
                            "choice" => parse_recipe_choice(
                                &children,
                                line.line_no,
                                &mut spec,
                                &recipe_segments,
                            ),
                            _ => unreachable!("named semantic block kind already validated"),
                        }
                    }
                }
                _ => {
                    return Err(KotobaError::Parse {
                        line: line.line_no,
                        message: format!("bloco removido ou desconhecido: {}", text),
                    })
                }
            }
            i = next;
            continue;
        }

        return Err(KotobaError::Parse {
            line: line.line_no,
            message: format!("comando Recipe desconhecido: {}", text),
        });
    }

    Ok(spec)
}

fn recipe_lines(source: &str) -> Vec<RecipeLine> {
    let mut out = Vec::new();
    for (idx, raw) in source.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed_start = raw.trim_start();
        if trimmed_start.is_empty() || trimmed_start.starts_with('#') {
            continue;
        }
        let indent = raw.len().saturating_sub(trimmed_start.len());
        let text = trimmed_start.trim_end().to_string();
        out.push(RecipeLine {
            line_no,
            indent,
            text,
        });
    }
    out
}

fn recipe_child_lines(
    lines: &[RecipeLine],
    mut i: usize,
    parent_indent: usize,
) -> (Vec<RecipeLine>, usize) {
    let mut out = Vec::new();
    while i < lines.len() {
        if lines[i].indent <= parent_indent {
            break;
        }
        out.push(lines[i].clone());
        i += 1;
    }
    (out, i)
}

fn validate_canonical_recipe(lines: &[RecipeLine]) -> Result<(), KotobaError> {
    if lines.is_empty() {
        return Ok(());
    }
    let root_indent = lines[0].indent;
    let mut i = 1usize;
    while i < lines.len() {
        let line = &lines[i];
        let text = line.text.trim().trim_end_matches(';').trim();
        if line.indent != root_indent + 4 {
            return Err(KotobaError::Parse {
                line: line.line_no,
                message: "indentação inválida: use quatro espaços por nível".into(),
            });
        }
        if is_canonical_recipe_root_statement(text) {
            i += 1;
            continue;
        }
        if text.ends_with(':') {
            let block = text.trim_end_matches(':').trim();
            if !is_canonical_recipe_block(block) {
                return Err(KotobaError::Parse {
                    line: line.line_no,
                    message: format!("bloco removido ou desconhecido: {}", text),
                });
            }
            let (children, next) = recipe_child_lines(lines, i + 1, line.indent);
            if children.is_empty() {
                return Err(KotobaError::Parse {
                    line: line.line_no,
                    message: format!("bloco vazio: {}", text),
                });
            }
            for child in &children {
                let command = child.text.trim().trim_end_matches(';').trim();
                if child.indent != line.indent + 4 {
                    return Err(KotobaError::Parse {
                        line: child.line_no,
                        message: format!("indentação inválida no bloco {}", block),
                    });
                }
                if !is_canonical_recipe_block_command(block, command) {
                    return Err(KotobaError::Parse {
                        line: child.line_no,
                        message: format!(
                            "comando removido ou não canônico em `{}`: {}",
                            block, command
                        ),
                    });
                }
            }
            i = next;
            continue;
        }
        return Err(KotobaError::Parse {
            line: line.line_no,
            message: format!("comando removido ou não canônico: {}", text),
        });
    }
    Ok(())
}

fn starts_with_value(line: &str, prefix: &str) -> bool {
    line.strip_prefix(prefix)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn is_canonical_recipe_root_statement(line: &str) -> bool {
    if starts_with_value(line, "file ") || starts_with_value(line, "encoding ") {
        return true;
    }
    if let Some(value) = line.strip_prefix("type ") {
        return value
            .split_once(" = matches ")
            .map(|(name, pattern)| !name.trim().is_empty() && !pattern.trim().is_empty())
            .unwrap_or(false)
            || value
                .split_once(" = like ")
                .map(|(name, pattern)| !name.trim().is_empty() && !pattern.trim().is_empty())
                .unwrap_or(false);
    }
    false
}

fn is_canonical_recipe_block(block: &str) -> bool {
    matches!(
        block,
        "quotes"
            | "protect"
            | "ignore"
            | "read"
            | "voice"
            | "speaker"
            | "dialogue"
            | "text"
            | "choice"
            | "numbered lines"
    ) || recipe_named_semantic_block(block).is_some()
        || block
            .strip_prefix("rule ")
            .map(|name| !name.trim().is_empty())
            .unwrap_or(false)
        || block
            .strip_prefix("json ")
            .map(|name| !name.trim().is_empty())
            .unwrap_or(false)
}

fn is_canonical_recipe_block_command(block: &str, line: &str) -> bool {
    if let Some((kind, _)) = recipe_named_semantic_block(block) {
        return if kind == "ignore" {
            is_canonical_recipe_semantic_rule_command(kind, line)
        } else {
            is_canonical_recipe_block_command(kind, line)
        };
    }
    if block == "quotes" {
        return starts_with_value(line, "pair ") && parse_all_quoted(line).len() == 2;
    }
    if block == "protect" {
        return (starts_with_value(line, "between ") && parse_all_quoted(line).len() == 2)
            || starts_with_value(line, "literal ")
            || starts_with_value(line, "matches ");
    }
    if block == "ignore" {
        return line == "empty"
            || line == "asset"
            || [
                "starts with any ",
                "starts with ",
                "contains any ",
                "contains ",
                "ends with any ",
                "ends with ",
                "equals any ",
                "equals ",
                "matches ",
                "like any ",
                "like ",
            ]
            .iter()
            .any(|prefix| starts_with_value(line, prefix));
    }
    if block == "read" {
        return matches!(
            line,
            "records as lines"
                | "records as blocks"
                | "records as segmented lines"
                | "records as binary"
                | "block ends when braces close"
        ) || [
            "block starts with ",
            "block ends with ",
            "segments separated by ",
            "choice segments separated by ",
            "fields ",
            "source field ",
            "patch field ",
            "magic ",
            "length ",
            "encoding ",
            "min length ",
            "profile ",
        ]
        .iter()
        .any(|prefix| starts_with_value(line, prefix));
    }
    if block == "voice" {
        return matches!(line, "remember voice" | "remember speaker" | "skip")
            || [
                "section ",
                "capture speaker from attribute ",
                "capture voice from attribute ",
                "when starts with ",
                "capture voice after ",
                "like any ",
                "like ",
                "matches ",
            ]
            .iter()
            .any(|prefix| starts_with_value(line, prefix));
    }
    if block == "speaker" {
        return matches!(
            line,
            "when previous is voice" | "when next is quoted" | "remember speaker" | "skip"
        ) || [
            "section ",
            "when starts with ",
            "capture speaker between ",
            "capture speaker from attribute ",
            "capture voice from attribute ",
        ]
        .iter()
        .any(|prefix| starts_with_value(line, prefix));
    }
    if block == "dialogue" {
        if line.starts_with("patch speaker") {
            return line == "patch speaker";
        }
        return [
            "section ",
            "field ",
            "when starts with ",
            "capture speaker between ",
            "capture text after ",
            "capture text between ",
            "capture voice if like ",
            "capture speaker before rest text if starts with any ",
            "capture text as ",
            "speaker fallback ",
            "save as ",
            "save otherwise as ",
            "patch ",
        ]
        .iter()
        .any(|prefix| starts_with_value(line, prefix))
            || is_canonical_recipe_semantic_rule_command("dialogue", line)
            || matches!(
                line,
                "capture id as number"
                    | "capture voice as optional"
                    | "capture speaker before quoted text"
                    | "patch speaker"
            );
    }
    if block == "text" {
        return [
            "field ",
            "section ",
            "capture text between ",
            "capture text as ",
            "save as ",
            "patch ",
        ]
        .iter()
        .any(|prefix| starts_with_value(line, prefix))
            || is_canonical_recipe_semantic_rule_command("text", line);
    }
    if block == "choice" {
        return [
            "field ",
            "when starts with ",
            "capture text as ",
            "save as ",
            "patch ",
        ]
        .iter()
        .any(|prefix| starts_with_value(line, prefix))
            || is_canonical_recipe_semantic_rule_command("choice", line)
            || line == "capture choices as quoted";
    }
    if block == "numbered lines" {
        return starts_with_value(line, "id between ") && parse_all_quoted(line).len() == 2;
    }
    if block.starts_with("json ") {
        return ["entries ", "text ", "speaker ", "context ", "id "]
            .iter()
            .any(|prefix| starts_with_value(line, prefix));
    }
    if block.starts_with("rule ") {
        if line.starts_with("patch speaker") {
            return line == "patch speaker";
        }
        return [
            "when matches ",
            "when format ",
            "when starts with ",
            "when exists ",
            "when not exists ",
            "capture ",
            "save as ",
            "patch ",
            "speaker fallback ",
            "context is ",
            "remember ",
            "forget ",
        ]
        .iter()
        .any(|prefix| starts_with_value(line, prefix))
            || matches!(
                line,
                "when previous is voice"
                    | "when previous is speaker"
                    | "when next is quoted"
                    | "when text is quoted"
                    | "patch speaker"
                    | "skip"
            );
    }
    false
}

fn recipe_named_semantic_block(block: &str) -> Option<(&str, &str)> {
    for kind in ["dialogue", "text", "choice", "ignore"] {
        let Some(name) = block
            .strip_prefix(kind)
            .and_then(|rest| rest.strip_prefix(' '))
        else {
            continue;
        };
        let name = name.trim();
        // `text block:` was a removed structural alias before semantic blocks
        // gained optional names. Do not silently reinterpret that legacy
        // spelling as a semantic text rule named `block`.
        if kind == "text" && name == "block" {
            continue;
        }
        if !name.is_empty() && !name.chars().any(char::is_whitespace) {
            return Some((kind, name));
        }
    }
    None
}

fn is_canonical_recipe_semantic_rule_command(kind: &str, line: &str) -> bool {
    if line == "patch speaker" {
        return kind == "dialogue";
    }
    if matches!(
        line,
        "when previous is voice"
            | "when previous is speaker"
            | "when next is quoted"
            | "when text is quoted"
            | "skip"
    ) {
        return true;
    }
    let common = [
        "when matches ",
        "when format ",
        "when starts with ",
        "when exists ",
        "when not exists ",
        "capture ",
        "save as ",
        "patch ",
        "remember ",
        "forget ",
        "context is ",
    ];
    if common.iter().any(|prefix| starts_with_value(line, prefix)) {
        return true;
    }
    kind == "dialogue"
        && ["speaker fallback ", "speaker is ", "text is "]
            .iter()
            .any(|prefix| starts_with_value(line, prefix))
}

fn recipe_uses_flexible_semantic_commands(children: &[RecipeLine]) -> bool {
    children.iter().any(|child| {
        let line = child.text.trim().trim_end_matches(';').trim();
        line == "when previous is voice"
            || line == "when previous is speaker"
            || line == "when next is quoted"
            || line == "when text is quoted"
            || line == "skip"
            || [
                "when matches ",
                "when format ",
                "when exists ",
                "when not exists ",
                "remember ",
                "forget ",
                "context is ",
                "speaker is ",
                "text is ",
            ]
            .iter()
            .any(|prefix| starts_with_value(line, prefix))
    })
}

fn parse_recipe_value(value: &str) -> String {
    parse_first_quoted(value)
        .unwrap_or_else(|| value.trim().trim_end_matches(';').trim().to_string())
}

fn parse_recipe_type_inline(
    value: &str,
    line_no: usize,
    spec: &mut KotobaParserSpec,
) -> Result<(), KotobaError> {
    let Some((name, rhs)) = value.split_once('=') else {
        return Err(KotobaError::Parse {
            line: line_no,
            message: "type Recipe deve usar: type Nome = regex/like ...".into(),
        });
    };
    let name = name.trim();
    let rhs = rhs.trim();
    let mut typ = KotobaTypeSpec {
        name: name.to_string(),
        ..KotobaTypeSpec::default()
    };
    if let Some(v) = rhs.strip_prefix("matches") {
        typ.patterns.push(parse_recipe_value(v));
    } else if let Some(v) = rhs.strip_prefix("like") {
        typ.patterns
            .push(recipe_like_to_regex(&parse_recipe_value(v)));
    } else {
        typ.patterns.push(parse_recipe_value(rhs));
    }
    spec.types.insert(name.to_string(), typ);
    Ok(())
}

fn parse_recipe_quotes(children: &[RecipeLine], spec: &mut KotobaParserSpec) {
    for child in children {
        let line = child.text.trim();
        if let Some(rest) = line.strip_prefix("pair ") {
            let values = parse_all_quoted(rest);
            if values.len() >= 2 {
                spec.quote_pairs
                    .push((values[0].clone(), values[1].clone()));
            }
            continue;
        }
        if line.contains(" to ") {
            let values = parse_all_quoted(line);
            if values.len() >= 2 {
                spec.quote_pairs
                    .push((values[0].clone(), values[1].clone()));
            }
        }
    }
}

fn parse_recipe_protect(children: &[RecipeLine], spec: &mut KotobaParserSpec) {
    for child in children {
        spec.protect.extend(recipe_protect_items(&child.text));
    }
}

fn recipe_protect_items(value: &str) -> Vec<KotobaProtectRule> {
    let line = value.trim().trim_end_matches(';').trim();
    if line.eq_ignore_ascii_case("slash commands") {
        return parse_simple_protect_items("slash_commands");
    }
    if line.eq_ignore_ascii_case("hash numbers") {
        return parse_simple_protect_items("hash_numbers");
    }
    if line.eq_ignore_ascii_case("html tags") {
        return parse_simple_protect_items("html_tags");
    }
    if line.eq_ignore_ascii_case("ruby tags") {
        return parse_simple_protect_items("ruby_tags");
    }
    if line.eq_ignore_ascii_case("angle tags") {
        return parse_simple_protect_items("angle_tags");
    }
    if line.eq_ignore_ascii_case("newlines") {
        return parse_simple_protect_items("newlines");
    }
    if let Some(value) = line.strip_prefix("literal ") {
        return vec![KotobaProtectRule::Literal(parse_recipe_value(value))];
    }
    if let Some(value) = line.strip_prefix("matches ") {
        return vec![KotobaProtectRule::Pattern(parse_recipe_value(value))];
    }
    if line.starts_with("between ")
        || line.starts_with("inline tags between ")
        || line.starts_with("tags between ")
    {
        let values = parse_all_quoted(line);
        if values.len() >= 2 {
            return parse_simple_protect_items(&format!(
                "quote \"{}\" \"{}\"",
                escape_recipe_quote(&values[0]),
                escape_recipe_quote(&values[1])
            ));
        }
    }
    if line.starts_with("brackets ") || line.starts_with("bracket ") {
        let values = parse_all_quoted(line);
        if values.len() >= 2 {
            return parse_simple_protect_items(&format!(
                "bracket \"{}\" \"{}\"",
                escape_recipe_quote(&values[0]),
                escape_recipe_quote(&values[1])
            ));
        }
    }
    parse_simple_protect_items(line)
}

fn parse_recipe_ignore(children: &[RecipeLine], spec: &mut KotobaParserSpec) {
    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if line.is_empty() {
            continue;
        }

        if line == "empty" {
            spec.skip_rules.push(KotobaSkipRule::Empty);
        } else if line == "asset" {
            spec.skip_rules.push(KotobaSkipRule::Asset);
        } else if let Some(value) = line.strip_prefix("starts with any ") {
            for item in parse_simple_values(value) {
                spec.skip_rules.push(KotobaSkipRule::StartsWith(item));
            }
        } else if let Some(value) = line.strip_prefix("starts with ") {
            spec.skip_rules
                .push(KotobaSkipRule::StartsWith(parse_recipe_value(value)));
        } else if let Some(value) = line.strip_prefix("contains any ") {
            for item in parse_simple_values(value) {
                spec.skip_rules.push(KotobaSkipRule::Contains(item));
            }
        } else if let Some(value) = line.strip_prefix("contains ") {
            spec.skip_rules
                .push(KotobaSkipRule::Contains(parse_recipe_value(value)));
        } else if let Some(value) = line.strip_prefix("ends with any ") {
            for item in parse_simple_values(value) {
                spec.skip_rules.push(KotobaSkipRule::EndsWith(item));
            }
        } else if let Some(value) = line.strip_prefix("ends with ") {
            spec.skip_rules
                .push(KotobaSkipRule::EndsWith(parse_recipe_value(value)));
        } else if let Some(value) = line.strip_prefix("equals any ") {
            for item in parse_simple_values(value) {
                spec.skip_rules.push(KotobaSkipRule::Equals(item));
            }
        } else if let Some(value) = line.strip_prefix("equals ") {
            spec.skip_rules
                .push(KotobaSkipRule::Equals(parse_recipe_value(value)));
        } else if let Some(value) = line.strip_prefix("matches ") {
            spec.skip_rules
                .push(KotobaSkipRule::Matching(parse_recipe_value(value)));
        } else if let Some(value) = line.strip_prefix("like any ") {
            for item in parse_simple_values(value) {
                spec.skip_rules
                    .push(KotobaSkipRule::Matching(recipe_like_to_regex(&item)));
            }
        } else if let Some(value) = line.strip_prefix("like ") {
            spec.skip_rules
                .push(KotobaSkipRule::Matching(recipe_like_to_regex(
                    &parse_recipe_value(value),
                )));
        }
    }
}

fn recipe_section_prefix(section: &str) -> String {
    format!(r#"(?s:.*?\b{}\s*=\s*\{{"#, regex::escape(section))
}

fn recipe_attr_capture(attr: &str, field: &str) -> String {
    format!(
        r#".*?\b{}\s*=\s*[\"']?(?P<{}>[^\"'\s,}}>]+)[\"']?"#,
        regex::escape(attr),
        field
    )
}

fn recipe_section_text_between_regex(section: &str, open: &str, close: &str) -> String {
    format!(
        r#"regex:{}.*?{}(?P<text>.*?){}.*)"#,
        recipe_section_prefix(section),
        regex::escape(open),
        regex::escape(close)
    )
}

fn recipe_section_quoted_regex(section: &str) -> String {
    // `text is quoted` means that the section value itself is quoted. Keep
    // the capture anchored immediately after the section's optional record
    // wrapper instead of searching for a quote anywhere in the block. Apart
    // from duplicating wrapped values such as [["Dialogue"]], the old `.*?`
    // search could continue past the requested section and capture a quoted
    // value from a following language section.
    format!(
        r#"regex:{}\s*(?:\{{\s*)?(?P<text>\"(?:\\.|[^\"])*\"|“[^”]*”|‘[^’]*’|≪[^≫]*≫|「[^」]*」|挌.*?拮|抛.*?拉|Åg.*?Åh|�g.*?�h).*)"#,
        recipe_section_prefix(section)
    )
}

fn recipe_section_attribute_rule(
    name: &str,
    section: &str,
    speaker_attr: Option<&str>,
    voice_attr: Option<&str>,
    source_line: Option<usize>,
) -> KotobaRule {
    let mut pattern = recipe_section_prefix(section);
    if let Some(attr) = speaker_attr {
        pattern.push_str(&recipe_attr_capture(attr, "speaker"));
    }
    if let Some(attr) = voice_attr {
        pattern.push_str(&recipe_attr_capture(attr, "voice"));
    }
    pattern.push_str(r#".*)"#);
    let mut rule = skip_rule(
        name,
        &format!("regex:{}", pattern.trim_start_matches("regex:")),
        source_line,
    );
    rule.extra_fields
        .insert("__type:speaker".into(), "line".into());
    rule.extra_fields
        .insert("__type:voice".into(), "line".into());
    rule
}

fn parse_recipe_voice(children: &[RecipeLine], spec: &mut KotobaParserSpec) {
    let mut patterns: Vec<String> = Vec::new();
    let mut remember_as_voice = false;
    let mut skip_line = false;
    let mut line_prefix: Option<String> = None;
    let mut voice_after: Option<String> = None;
    let mut section = RecipeSectionCaptureSpec::default();

    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if let Some(value) = line.strip_prefix("section ") {
            section.section = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("take speaker from attribute ")
            .or_else(|| line.strip_prefix("capture speaker from attribute "))
            .or_else(|| line.strip_prefix("speaker from attribute "))
        {
            section.speaker_attr = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("take voice from attribute ")
            .or_else(|| line.strip_prefix("capture voice from attribute "))
            .or_else(|| line.strip_prefix("voice from attribute "))
        {
            section.voice_attr = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("when starts with ") {
            line_prefix = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("take voice after ")
            .or_else(|| line.strip_prefix("capture voice after "))
            .or_else(|| line.strip_prefix("voice after "))
        {
            voice_after = Some(parse_recipe_separator(value));
        } else if let Some(value) = line.strip_prefix("format ") {
            patterns.push(recipe_like_to_regex(&parse_recipe_value(value)));
        } else if let Some(value) = line
            .strip_prefix("content like any ")
            .or_else(|| line.strip_prefix("line like any "))
            .or_else(|| line.strip_prefix("like any "))
        {
            for item in parse_simple_values(value) {
                patterns.push(recipe_like_to_regex(&item));
            }
        } else if let Some(value) = line
            .strip_prefix("content like ")
            .or_else(|| line.strip_prefix("line like "))
            .or_else(|| line.strip_prefix("like "))
        {
            patterns.push(recipe_like_to_regex(&parse_recipe_value(value)));
        } else if let Some(value) = line
            .strip_prefix("content matching ")
            .or_else(|| line.strip_prefix("content matches "))
            .or_else(|| line.strip_prefix("line matching "))
            .or_else(|| line.strip_prefix("matching "))
            .or_else(|| line.strip_prefix("matches "))
        {
            patterns.push(parse_recipe_value(value));
        } else if line == "remember voice" {
            remember_as_voice = true;
            section.remember_voice = true;
        } else if line == "remember speaker" {
            section.remember_speaker = true;
        } else if line == "skip" {
            skip_line = true;
            section.skip = true;
        }
    }

    if let Some(section_name) = section.section.as_deref() {
        let mut rule = recipe_section_attribute_rule(
            "remember_section_voice",
            section_name,
            section.speaker_attr.as_deref(),
            section.voice_attr.as_deref(),
            None,
        );
        if section.remember_voice || remember_as_voice {
            if section.voice_attr.is_some() {
                rule.remember.push("voice".into());
            }
            if section.speaker_attr.is_some() {
                rule.remember.push("speaker".into());
            }
        }
        if section.remember_speaker
            && section.speaker_attr.is_some()
            && !rule.remember.contains(&"speaker".into())
        {
            rule.remember.push("speaker".into());
        }
        spec.rules.push(rule);
        return;
    }

    if !patterns.is_empty() {
        spec.types.insert(
            "VoiceId".into(),
            KotobaTypeSpec {
                name: "VoiceId".into(),
                patterns,
                values: Vec::new(),
                trim: true,
            },
        );
    }
    if let (Some(prefix), Some(after)) = (line_prefix.as_deref(), voice_after.as_deref()) {
        let mut rule = skip_rule(
            "remember_voice_after",
            &format!(
                "regex:{}.*?{}(?P<voice>[^\r\n]+).*",
                regex::escape(prefix),
                regex::escape(after)
            ),
            None,
        );
        rule.extra_fields
            .insert("__type:voice".into(), "line".into());
        if remember_as_voice {
            rule.forget.push("speaker".into());
            rule.remember.push("voice".into());
        }
        spec.rules.push(rule);
        return;
    }
    if remember_as_voice || skip_line {
        if !spec.types.contains_key("VoiceId") {
            spec.types.insert(
                "VoiceId".into(),
                KotobaTypeSpec {
                    name: "VoiceId".into(),
                    patterns: vec![r"^[A-Za-z0-9_]+_[0-9]{4,}(?:_[A-Za-z0-9_]+)?$".into()],
                    values: Vec::new(),
                    trim: true,
                },
            );
        }
        let mut rule = skip_rule("remember_voice", "<voice:VoiceId>", None);
        if remember_as_voice {
            // A new voice id normally starts a new spoken line. Clear a stale remembered speaker
            // so `dialogue: speaker fallback voice.speaker` does not reuse the previous character.
            rule.forget.push("speaker".into());
            rule.remember.push("voice".into());
        }
        spec.rules.push(rule);
    }
}

fn parse_recipe_speaker(children: &[RecipeLine], source_line: usize, spec: &mut KotobaParserSpec) {
    let mut require_voice = false;
    let mut next_is_quoted = false;
    let mut remember_as_speaker = false;
    let mut skip_line = false;
    let mut line_prefix: Option<String> = None;
    let mut capture_between: Option<(String, String)> = None;
    let mut capture_attribute: Option<String> = None;
    let mut section: Option<String> = None;
    let mut voice_attribute: Option<String> = None;

    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if line == "when previous is voice" {
            require_voice = true;
        } else if line == "when next is quoted" {
            next_is_quoted = true;
        } else if let Some(value) = line.strip_prefix("section ") {
            section = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("when starts with ") {
            line_prefix = Some(parse_recipe_value(value));
        } else if line.starts_with("take speaker between ")
            || line.starts_with("capture speaker between ")
        {
            let values = parse_all_quoted(line);
            if values.len() >= 2 {
                capture_between = Some((values[0].clone(), values[1].clone()));
            }
        } else if let Some(value) = line
            .strip_prefix("take speaker from attribute ")
            .or_else(|| line.strip_prefix("capture speaker from attribute "))
            .or_else(|| line.strip_prefix("speaker from attribute "))
        {
            capture_attribute = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("take voice from attribute ")
            .or_else(|| line.strip_prefix("capture voice from attribute "))
            .or_else(|| line.strip_prefix("voice from attribute "))
        {
            voice_attribute = Some(parse_recipe_value(value));
        } else if line == "remember speaker" {
            remember_as_speaker = true;
        } else if line == "skip" {
            skip_line = true;
        }
    }

    spec.types
        .entry("SpeakerName".into())
        .or_insert_with(|| KotobaTypeSpec {
            name: "SpeakerName".into(),
            // Speaker lines in numbered scripts are plain labels like `Yuki` or `Takuji`.
            // Do not accept quoted dialogue as a speaker candidate.
            patterns: vec![r#"^[^"“”‘’≪≫「」].*\S.*$"#.into()],
            values: Vec::new(),
            trim: true,
        });

    if let Some(section_name) = section.as_deref() {
        let mut rule = recipe_section_attribute_rule(
            "remember_section_speaker",
            section_name,
            capture_attribute.as_deref(),
            voice_attribute.as_deref(),
            Some(source_line),
        );
        if remember_as_speaker && capture_attribute.is_some() {
            rule.remember.push("speaker".into());
        }
        if voice_attribute.is_some() {
            rule.remember.push("voice".into());
        }
        spec.rules.push(rule);
        return;
    }

    if capture_between.is_some() || capture_attribute.is_some() {
        let mut rule = if let Some(attr) = capture_attribute {
            let prefix = line_prefix
                .as_deref()
                .map(regex::escape)
                .unwrap_or_else(|| ".*?".into());
            let mut pattern = prefix;
            pattern.push_str(&recipe_attr_capture(&attr, "speaker"));
            if let Some(voice_attr) = voice_attribute.as_deref() {
                pattern.push_str(&recipe_attr_capture(voice_attr, "voice"));
            }
            pattern.push_str(".*");
            skip_rule(
                "remember_speaker_attribute",
                &format!("regex:{}", pattern),
                Some(source_line),
            )
        } else if let Some((open, close)) = capture_between {
            let prefix = match line_prefix.as_deref() {
                Some(prefix) if open.starts_with(prefix) || prefix.starts_with(&open) => {
                    String::new()
                }
                Some(prefix) => format!("{}.*?", regex::escape(prefix)),
                None => ".*?".into(),
            };
            skip_rule(
                "remember_speaker_between",
                &format!(
                    r#"regex:{}{}(?P<speaker>.*?){}.*"#,
                    prefix,
                    regex::escape(&open),
                    regex::escape(&close)
                ),
                Some(source_line),
            )
        } else {
            skip_rule(
                "remember_speaker",
                "<speaker:SpeakerName>",
                Some(source_line),
            )
        };
        rule.extra_fields
            .insert("__type:speaker".into(), "line".into());
        if remember_as_speaker {
            rule.remember.push("speaker".into());
        }
        if voice_attribute.is_some() {
            rule.remember.push("voice".into());
        }
        spec.rules.push(rule);
        return;
    }

    if remember_as_speaker || skip_line {
        let mut rule = skip_rule(
            "remember_speaker",
            "<speaker:SpeakerName>",
            Some(source_line),
        );
        if require_voice {
            rule.when.push(KotobaCondition::Exists {
                name: "voice".into(),
            });
        }
        if next_is_quoted {
            rule.next_pattern = Some("<text:quoted>".into());
        }
        if remember_as_speaker {
            rule.remember.push("speaker".into());
        }
        spec.rules.push(rule);
    }
}

fn parse_recipe_dialogue(
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &RecipeSegmentsSpec,
) {
    let mut text_type = "quoted".to_string();
    let mut entry_type = "Dialogue".to_string();
    let mut patch_field = "text".to_string();
    let mut fallback_remembered_speaker = false;
    let mut fallback_voice_speaker = false;
    let mut line = RecipeDialogueLineSpec {
        voice_type: "VoiceId".into(),
        text_mode: "quoted".into(),
        patch_field: "text".into(),
        entry_type: "Dialogue".into(),
        narration_type: "Narration".into(),
        ..RecipeDialogueLineSpec::default()
    };

    for child in children {
        let raw = child.text.trim().trim_end_matches(';').trim();
        if let Some(value) = raw.strip_prefix("section ") {
            line.section = Some(parse_recipe_value(value));
        } else if let Some(value) = raw
            .strip_prefix("segment ")
            .or_else(|| raw.strip_prefix("language "))
            .or_else(|| raw.strip_prefix("field "))
        {
            line.segment = Some(sanitize_id(&parse_recipe_value(value)));
        } else if raw.starts_with("speaker between ")
            || raw.starts_with("take speaker between ")
            || raw.starts_with("capture speaker between ")
        {
            let values = parse_all_quoted(raw);
            if values.len() >= 2 {
                line.speaker_between = Some((values[0].clone(), values[1].clone()));
            }
        } else if let Some(value) = raw.strip_prefix("capture text after ") {
            line.text_after = Some(parse_recipe_value(value));
        } else if raw.starts_with("capture text between ") {
            let values = parse_all_quoted(raw);
            if values.len() >= 2 {
                line.text_between = Some((values[0].clone(), values[1].clone()));
            }
        } else if let Some(value) = raw.strip_prefix("when starts with ") {
            line.command = parse_recipe_value(value);
        } else if let Some(value) = raw
            .strip_prefix("from line after ")
            .or_else(|| raw.strip_prefix("from content after "))
        {
            line.command = parse_recipe_value(value);
        } else if raw == "id is number"
            || raw == "take id as number"
            || raw == "capture id as number"
        {
            line.has_id = true;
        } else if raw == "capture voice as optional" {
            line.has_voice = true;
            if !spec.types.contains_key(&line.voice_type) {
                spec.types.insert(
                    line.voice_type.clone(),
                    KotobaTypeSpec {
                        name: line.voice_type.clone(),
                        patterns: vec![r"^[A-Za-z]{3}-[^\s-]+-[0-9]{4}$".into()],
                        values: Vec::new(),
                        trim: true,
                    },
                );
            }
        } else if let Some(value) = raw.strip_prefix("capture voice if like ") {
            line.has_voice = true;
            line.voice_type = "VoiceId".into();
            spec.types.insert(
                line.voice_type.clone(),
                KotobaTypeSpec {
                    name: line.voice_type.clone(),
                    patterns: vec![recipe_like_to_regex(&parse_recipe_value(value))],
                    values: Vec::new(),
                    trim: true,
                },
            );
        } else if raw == "take speaker before quoted text"
            || raw == "capture speaker before quoted text"
            || raw == "speaker before quoted text"
            || raw == "speaker is before text"
        {
            line.plain_speaker_before_quoted = true;
        } else if raw.starts_with("take speaker before rest text if starts with any ")
            || raw.starts_with("capture speaker before rest text if starts with any ")
            || raw.starts_with("speaker before rest text if starts with any ")
        {
            let value = raw
                .split_once(" starts with any ")
                .map(|(_, rhs)| rhs)
                .unwrap_or_default();
            for item in parse_simple_values(value) {
                if !line.marked_speaker_prefixes.contains(&item) {
                    line.marked_speaker_prefixes.push(item);
                }
            }
        } else if raw.starts_with("speaker removes ")
            || raw.starts_with("speaker strips ")
            || raw.starts_with("strip speaker ")
        {
            let value = raw.split_once(' ').map(|(_, rhs)| rhs).unwrap_or_default();
            for item in parse_simple_values(value) {
                if !line.marked_speaker_prefixes.contains(&item) {
                    line.marked_speaker_prefixes.push(item);
                }
            }
        } else if let Some(value) = raw.strip_prefix("capture text as ") {
            let value = value.trim();
            text_type = match value {
                "content" | "rest" => "rest".into(),
                "quoted" => "quoted".into(),
                other => other.to_string(),
            };
            line.text_mode = value.to_string();
        } else if let Some(value) = raw.strip_prefix("save otherwise as ") {
            line.emit_narration_otherwise = true;
            line.narration_type = parse_recipe_value(value);
        } else if let Some(value) = raw.strip_prefix("save as ") {
            let value = value.trim();
            if value.eq_ignore_ascii_case("Dialogue if speaker exists") {
                line.entry_type = "Dialogue".into();
            } else if value.eq_ignore_ascii_case("Narration otherwise") {
                line.emit_narration_otherwise = true;
                line.narration_type = "Narration".into();
            } else {
                entry_type = parse_recipe_value(value);
                line.entry_type = entry_type.clone();
            }
        } else if raw == "patch speaker" {
            line.speaker_patch_field = Some("speaker".into());
        } else if let Some(value) = raw.strip_prefix("patch ") {
            let value = parse_recipe_value(value);
            patch_field = if value == "content" {
                "text".into()
            } else {
                value.clone()
            };
            line.patch_field = if value == "content" {
                "text".into()
            } else {
                value
            };
        } else if raw == "speaker fallback remembered speaker"
            || raw == "speaker fallback speaker"
            || raw == "speaker falls back to remembered speaker"
        {
            fallback_remembered_speaker = true;
        } else if raw == "speaker fallback voice.speaker"
            || raw == "speaker falls back to voice.speaker"
        {
            fallback_voice_speaker = true;
        }
    }

    if line.segment.is_some() && segments.has_text_segments() {
        compile_recipe_segmented_dialogue_rule(&line, source_line, spec, segments);
        return;
    }

    if line.section.is_some() {
        compile_recipe_section_dialogue_rule(
            line,
            fallback_remembered_speaker,
            fallback_voice_speaker,
            source_line,
            spec,
        );
        return;
    }

    if line.speaker_between.is_some() && line.text_after.is_some() {
        compile_recipe_structural_dialogue_rule(line, source_line, spec);
        return;
    }

    if !line.command.is_empty() {
        compile_recipe_dialogue_line_rules(line, fallback_voice_speaker, source_line, spec);
        return;
    }

    if fallback_remembered_speaker {
        let mut rule = entry_rule(
            "dialogue_speaker",
            &format!("<text:{}>", text_type),
            &entry_type,
            "text",
            Some("speaker"),
            Some("voice"),
            &patch_field,
            None,
            source_line,
        );
        rule.when.push(KotobaCondition::Exists {
            name: "speaker".into(),
        });
        spec.rules.push(rule);
    }
    if fallback_voice_speaker {
        let mut rule = entry_rule(
            "dialogue_voice_speaker",
            &format!("<text:{}>", text_type),
            &entry_type,
            "text",
            Some("voice.speaker"),
            Some("voice"),
            &patch_field,
            None,
            source_line,
        );
        rule.when.push(KotobaCondition::Exists {
            name: "voice".into(),
        });
        rule.when.push(KotobaCondition::NotExists {
            name: "speaker".into(),
        });
        spec.rules.push(rule);
    }
    if !fallback_remembered_speaker && !fallback_voice_speaker {
        spec.rules.push(entry_rule(
            "dialogue",
            &format!("<text:{}>", text_type),
            &entry_type,
            "text",
            None,
            Some("line_number"),
            &patch_field,
            None,
            source_line,
        ));
    }
}

fn compile_recipe_structural_dialogue_rule(
    line: RecipeDialogueLineSpec,
    source_line: usize,
    spec: &mut KotobaParserSpec,
) {
    let mut flex = RecipeFlexibleRuleSpec {
        name: kind_to_snake(&line.entry_type),
        source_line,
        line_prefix: (!line.command.is_empty()).then_some(line.command),
        capture_between: line
            .speaker_between
            .map(|(open, close)| ("speaker".into(), open, close)),
        capture_after: line.text_after.map(|delimiter| ("text".into(), delimiter)),
        entry_type: Some(line.entry_type),
        text_field: Some("text".into()),
        // A speaker captured on the current line always takes precedence over
        // remembered context and voice metadata.
        speaker_field: Some("speaker".into()),
        patch_field: Some(line.patch_field),
        speaker_patch_field: line.speaker_patch_field,
        ..RecipeFlexibleRuleSpec::default()
    };
    flex.capture_types.insert("speaker".into(), "line".into());
    flex.capture_types.insert("text".into(), "line".into());

    let rule = compile_recipe_flexible_rule(flex)
        .expect("a structural dialogue built by the canonical Recipe must compile");
    spec.rules.push(rule);
}

fn compile_recipe_section_dialogue_rule(
    mut line: RecipeDialogueLineSpec,
    fallback_remembered_speaker: bool,
    fallback_voice_speaker: bool,
    source_line: usize,
    spec: &mut KotobaParserSpec,
) {
    let Some(section) = line.section.clone() else {
        return;
    };
    if line.patch_field.trim().is_empty() {
        line.patch_field = "text".into();
    }
    if line.entry_type.trim().is_empty() {
        line.entry_type = "Dialogue".into();
    }
    let pattern = if let Some((open, close)) = &line.text_between {
        recipe_section_text_between_regex(&section, open, close)
    } else if recipe_text_types(&line.text_mode).contains(&"quoted") {
        recipe_section_quoted_regex(&section)
    } else {
        format!(
            r#"regex:{}.*?(?P<text>[^\n\r]+).*)"#,
            recipe_section_prefix(&section)
        )
    };
    let mut rule = entry_rule(
        &format!(
            "{}_section_{}",
            kind_to_snake(&line.entry_type),
            sanitize_id(&section)
        ),
        &pattern,
        &line.entry_type,
        "text",
        None,
        Some("voice"),
        &line.patch_field,
        line.speaker_patch_field.as_deref(),
        source_line,
    );
    rule.extra_fields.insert(
        "__type:text".into(),
        if line.text_between.is_some() {
            "line".into()
        } else {
            "quoted".into()
        },
    );
    if fallback_remembered_speaker {
        rule.speaker_field = Some("speaker".into());
    }
    if fallback_voice_speaker && rule.speaker_field.is_none() {
        rule.speaker_field = Some("voice.speaker".into());
    }
    spec.rules.push(rule);
}

fn compile_recipe_dialogue_line_rules(
    mut line: RecipeDialogueLineSpec,
    fallback_voice_speaker: bool,
    source_line: usize,
    spec: &mut KotobaParserSpec,
) {
    if line.patch_field.trim().is_empty() {
        line.patch_field = "text".into();
    }
    if line.entry_type.trim().is_empty() {
        line.entry_type = "Dialogue".into();
    }
    if line.narration_type.trim().is_empty() {
        line.narration_type = "Narration".into();
    }
    if line.voice_type.trim().is_empty() {
        line.voice_type = "VoiceId".into();
    }
    if line.has_voice && !spec.types.contains_key(&line.voice_type) {
        spec.types.insert(
            line.voice_type.clone(),
            KotobaTypeSpec {
                name: line.voice_type.clone(),
                patterns: vec![r"^[A-Za-z]{3}-[^\s-]+-[0-9]{4}$".into()],
                values: Vec::new(),
                trim: true,
            },
        );
    }

    let text_types = recipe_text_types(&line.text_mode);
    let voice_variants: Vec<bool> = if line.has_voice {
        vec![true, false]
    } else {
        vec![false]
    };

    for with_voice in &voice_variants {
        let base = recipe_dialogue_command_base(&line, *with_voice);
        if base.is_empty() {
            continue;
        }

        for prefix in &line.marked_speaker_prefixes {
            for text_type in &text_types {
                spec.rules.push(entry_rule(
                    &format!(
                        "dialogue_line_{}_marked_{}_{}",
                        if *with_voice { "voice" } else { "plain" },
                        sanitize_id(prefix),
                        text_type
                    ),
                    &format!("{} {}<speaker:word> <text:{}>", base, prefix, text_type),
                    &line.entry_type,
                    "text",
                    Some("speaker"),
                    if *with_voice {
                        Some("voice")
                    } else {
                        Some("id")
                    },
                    &line.patch_field,
                    line.speaker_patch_field.as_deref(),
                    source_line,
                ));
            }
        }

        if line.plain_speaker_before_quoted {
            spec.rules.push(entry_rule(
                &format!(
                    "dialogue_line_{}_speaker_quoted",
                    if *with_voice { "voice" } else { "plain" }
                ),
                &format!("{} <speaker:word> <text:quoted>", base),
                &line.entry_type,
                "text",
                Some("speaker"),
                if *with_voice {
                    Some("voice")
                } else {
                    Some("id")
                },
                &line.patch_field,
                line.speaker_patch_field.as_deref(),
                source_line,
            ));
        }

        if *with_voice && fallback_voice_speaker {
            for text_type in &text_types {
                spec.rules.push(entry_rule(
                    &format!("dialogue_line_voice_fallback_{}", text_type),
                    &format!("{} <text:{}>", base, text_type),
                    &line.entry_type,
                    "text",
                    Some("voice.speaker"),
                    Some("voice"),
                    &line.patch_field,
                    None,
                    source_line,
                ));
            }
        }
    }

    if line.emit_narration_otherwise {
        for with_voice in &voice_variants {
            if *with_voice && fallback_voice_speaker {
                continue;
            }
            let base = recipe_dialogue_command_base(&line, *with_voice);
            if base.is_empty() {
                continue;
            }
            for text_type in &text_types {
                spec.rules.push(entry_rule(
                    &format!(
                        "dialogue_line_{}_fallback_{}",
                        if *with_voice { "voice" } else { "plain" },
                        text_type
                    ),
                    &format!("{} <text:{}>", base, text_type),
                    &line.narration_type,
                    "text",
                    None,
                    if *with_voice {
                        Some("voice")
                    } else {
                        Some("id")
                    },
                    &line.patch_field,
                    None,
                    source_line,
                ));
            }
        }
    }
}

fn recipe_dialogue_command_base(line: &RecipeDialogueLineSpec, with_voice: bool) -> String {
    let mut parts = Vec::new();
    parts.push(line.command.clone());
    if line.has_id {
        parts.push("<id:number>".into());
    }
    if with_voice {
        parts.push(format!("<voice:{}>", line.voice_type));
    }
    parts.join(" ")
}

fn recipe_text_types(text_mode: &str) -> Vec<&'static str> {
    let mode = text_mode.to_ascii_lowercase().replace('_', " ");
    if mode.contains("quoted") && mode.contains("rest") {
        vec!["quoted", "rest"]
    } else if mode.contains("quoted") {
        vec!["quoted"]
    } else {
        vec!["rest"]
    }
}

fn parse_recipe_read(
    children: &[RecipeLine],
    _source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &mut RecipeSegmentsSpec,
) {
    let mut binary_block: Option<KotobaBinaryBlockSpec> = None;
    let mut block_read: Option<RecipeBlockReadSpec> = None;
    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        let line_lower = line.to_ascii_lowercase();
        if line == "records as lines" || line == "lines" || line == "records lines" {
            // Line stream is the default runtime model.
        } else if line_lower == "records as blocks"
            || line_lower == "records as block"
            || line_lower == "blocks"
            || line_lower == "block records"
        {
            block_read.get_or_insert_with(RecipeBlockReadSpec::default);
        } else if let Some(value) = line
            .strip_prefix("block starts with ")
            .or_else(|| line.strip_prefix("record starts with "))
        {
            block_read
                .get_or_insert_with(RecipeBlockReadSpec::default)
                .start = parse_recipe_value(value);
        } else if let Some(value) = line
            .strip_prefix("block ends with ")
            .or_else(|| line.strip_prefix("record ends with "))
        {
            let parsed = parse_recipe_value(value);
            let block = block_read.get_or_insert_with(RecipeBlockReadSpec::default);
            if parsed.eq_ignore_ascii_case("matching brace")
                || parsed.eq_ignore_ascii_case("matching braces")
                || parsed.eq_ignore_ascii_case("balanced braces")
            {
                block.end = "__balanced_braces__".into();
                block.balanced_braces = true;
            } else {
                block.end = parsed;
            }
        } else if line_lower == "block ends when braces close"
            || line_lower == "record ends when braces close"
            || line_lower == "block ends at matching brace"
            || line_lower == "record ends at matching brace"
        {
            let block = block_read.get_or_insert_with(RecipeBlockReadSpec::default);
            block.end = "__balanced_braces__".into();
            block.balanced_braces = true;
        } else if let Some(value) = line
            .strip_prefix("json strings at ")
            .or_else(|| line.strip_prefix("records as json strings at "))
        {
            let entries = parse_recipe_value(value);
            spec.json_paths.push(KotobaJsonPathSpec {
                name: "json_strings".into(),
                entries,
                text: String::new(),
                speaker: None,
                context: None,
                id: None,
            });
        } else if line_lower == "records as segmented lines"
            || line_lower == "segmented lines"
            || line_lower == "records as segments"
            || line_lower == "segments"
        {
            // Segment metadata is consumed by dialogue/text/choice recipe blocks.
        } else if let Some(value) = line
            .strip_prefix("segments separated by ")
            .or_else(|| line.strip_prefix("segment separator "))
            .or_else(|| line.strip_prefix("dialogue separator "))
            .or_else(|| line.strip_prefix("text separator "))
        {
            segments.text_separator = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("choice segments separated by ")
            .or_else(|| line.strip_prefix("choices separated by "))
            .or_else(|| line.strip_prefix("choice separator "))
        {
            segments.choice_separator = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("fields ")
            .or_else(|| line.strip_prefix("languages "))
            .or_else(|| line.strip_prefix("columns "))
            .or_else(|| line.strip_prefix("segments named "))
        {
            segments.fields = parse_simple_values(value)
                .into_iter()
                .map(|v| sanitize_id(&v))
                .filter(|v| !v.is_empty())
                .collect();
        } else if let Some(value) = line
            .strip_prefix("source field ")
            .or_else(|| line.strip_prefix("source language "))
            .or_else(|| line.strip_prefix("source segment "))
        {
            segments.source_field = Some(sanitize_id(&parse_recipe_value(value)));
        } else if let Some(value) = line
            .strip_prefix("patch field ")
            .or_else(|| line.strip_prefix("patch language "))
            .or_else(|| line.strip_prefix("patch segment "))
        {
            segments.patch_field = Some(sanitize_id(&parse_recipe_value(value)));
        } else if line == "records as separated fields"
            || line == "separated fields"
            || line == "fields separated by tab"
        {
            // Field splitting is declared per rule for now so patch spans stay precise.
        } else if line_lower == "records as binary"
            || line_lower == "records as binary strings"
            || line_lower == "binary records"
            || line_lower == "binary records:"
            || line_lower == "binary strings"
            || line_lower == "binary strings:"
        {
            binary_block.get_or_insert_with(default_recipe_binary_block);
            spec.rebuild_strategy = Some(KotobaRebuildStrategySpec {
                mode: "binary".into(),
                on_tag_mismatch: "warn".into(),
                allow_line_growth: true,
            });
        } else if is_recipe_binary_line(line) {
            let block = binary_block.get_or_insert_with(default_recipe_binary_block);
            apply_recipe_binary_line(line, block);
            spec.rebuild_strategy = Some(KotobaRebuildStrategySpec {
                mode: "binary".into(),
                on_tag_mismatch: "warn".into(),
                allow_line_growth: true,
            });
        }
    }
    if let Some(block) = binary_block {
        spec.binary_blocks.push(block);
    }
    if let Some(block) = block_read {
        if !block.start.trim().is_empty() {
            spec.blocks.push(KotobaBlockSpec {
                name: "record_block".into(),
                start: block.start,
                end: if block.balanced_braces || block.end.trim().is_empty() {
                    "__balanced_braces__".into()
                } else {
                    block.end
                },
                rules: Vec::new(),
            });
        }
    }
}

#[derive(Debug, Clone, Default)]
struct RecipeFieldCapture {
    name: String,
    index: usize,
    separator: String,
}

#[derive(Debug, Clone, Default)]
struct RecipeFlexibleRuleSpec {
    name: String,
    source_line: usize,
    raw_regex: Option<String>,
    template_pattern: Option<String>,
    line_prefix: Option<String>,
    capture_between: Option<(String, String, String)>,
    capture_after: Option<(String, String)>,
    capture_before: Option<(String, String)>,
    capture_attribute: Option<(String, String)>,
    capture_as: BTreeMap<String, String>,
    field_captures: Vec<RecipeFieldCapture>,
    entry_type: Option<String>,
    text_field: Option<String>,
    speaker_field: Option<String>,
    context_field: Option<String>,
    patch_field: Option<String>,
    speaker_patch_field: Option<String>,
    remember: Vec<String>,
    forget: Vec<String>,
    when: Vec<KotobaCondition>,
    next_pattern: Option<String>,
    skip: bool,
    capture_types: BTreeMap<String, String>,
}

fn parse_recipe_flexible_rule(
    name: &str,
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
) -> Result<(), KotobaError> {
    let flex = RecipeFlexibleRuleSpec {
        name: sanitize_id(name),
        source_line,
        ..RecipeFlexibleRuleSpec::default()
    };
    parse_recipe_flexible_rule_from_spec(flex, children, source_line, spec, "rule")
}

fn parse_recipe_semantic_rule(
    kind: &str,
    name: Option<&str>,
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
) -> Result<(), KotobaError> {
    let default_name = format!("{}_l{}", kind, source_line);
    let mut flex = RecipeFlexibleRuleSpec {
        name: sanitize_id(name.unwrap_or(&default_name)),
        source_line,
        ..RecipeFlexibleRuleSpec::default()
    };
    match kind {
        "dialogue" => {
            flex.entry_type = Some("Dialogue".into());
            flex.text_field = Some("text".into());
        }
        "text" => {
            flex.entry_type = Some("Narration".into());
            flex.text_field = Some("text".into());
        }
        "choice" => {
            flex.entry_type = Some("Choice".into());
            flex.text_field = Some("text".into());
        }
        "ignore" => {
            flex.skip = true;
        }
        _ => {
            return Err(KotobaError::Parse {
                line: source_line,
                message: format!("bloco semântico desconhecido: {}", kind),
            });
        }
    }
    parse_recipe_flexible_rule_from_spec(flex, children, source_line, spec, kind)
}

fn parse_recipe_flexible_rule_from_spec(
    mut flex: RecipeFlexibleRuleSpec,
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
    block_kind: &str,
) -> Result<(), KotobaError> {
    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if line.is_empty() {
            continue;
        }

        if let Some(value) = line.strip_prefix("when matches ") {
            flex.raw_regex = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("when format ") {
            flex.template_pattern = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("when starts with ") {
            flex.line_prefix = Some(parse_recipe_value(value));
        } else if line == "when previous saved as voice" || line == "when previous is voice" {
            flex.when.push(KotobaCondition::Exists {
                name: "voice".into(),
            });
        } else if line == "when previous saved as speaker" || line == "when previous is speaker" {
            flex.when.push(KotobaCondition::Exists {
                name: "speaker".into(),
            });
        } else if let Some(value) = line.strip_prefix("when exists ") {
            flex.when.push(KotobaCondition::Exists {
                name: parse_recipe_value(value),
            });
        } else if let Some(value) = line.strip_prefix("when not exists ") {
            flex.when.push(KotobaCondition::NotExists {
                name: parse_recipe_value(value),
            });
        } else if line == "when next content is quoted"
            || line == "when next line is quoted"
            || line == "when next is quoted"
        {
            flex.next_pattern = Some("<text:quoted>".into());
        } else if line == "when text is quoted" {
            flex.capture_as.insert("text".into(), "quoted".into());
        } else if let Some(rest) = line.strip_prefix("capture ") {
            parse_recipe_flexible_capture(rest, &mut flex)?;
        } else if let Some(value) = line.strip_prefix("save as ") {
            flex.entry_type = Some(parse_recipe_value(value));
        } else if line == "patch speaker" {
            flex.speaker_patch_field = Some("speaker".into());
        } else if let Some(value) = line.strip_prefix("patch ") {
            flex.patch_field = Some(recipe_patch_field(value));
        } else if let Some(value) = line.strip_prefix("speaker fallback ") {
            flex.speaker_field = Some(recipe_speaker_fallback_field(value));
        } else if let Some(value) = line.strip_prefix("speaker is ") {
            flex.speaker_field = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("text is ") {
            let typ = recipe_text_type(value);
            flex.capture_as.insert("text".into(), typ);
        } else if let Some(value) = line.strip_prefix("context is ") {
            flex.context_field = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("remember ") {
            flex.remember.push(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("forget ") {
            flex.forget.push(parse_recipe_value(value));
        } else if line == "skip" {
            flex.skip = true;
        } else {
            return Err(KotobaError::Parse {
                line: child.line_no,
                message: format!("comando {} desconhecido: {}", block_kind, line),
            });
        }
    }

    let mut rule = compile_recipe_flexible_rule(flex)?;
    if rule.source_line.is_none() {
        rule.source_line = Some(source_line);
    }
    spec.rules.push(rule);
    Ok(())
}

fn recipe_patch_field(value: &str) -> String {
    let value = parse_recipe_value(value);
    if value == "content" || value == "value" {
        "text".into()
    } else {
        value
    }
}

fn recipe_speaker_fallback_field(value: &str) -> String {
    let value = parse_recipe_value(value).to_ascii_lowercase();
    match value.as_str() {
        "remembered speaker" | "speaker" => "speaker".into(),
        "voice.speaker" => "voice.speaker".into(),
        _ => value,
    }
}

fn recipe_text_type(value: &str) -> String {
    let value = parse_recipe_value(value).to_ascii_lowercase();
    match value.as_str() {
        "content" | "line" | "value" | "rest" => "rest".into(),
        "quoted" => "quoted".into(),
        other => other.to_string(),
    }
}

fn parse_recipe_flexible_capture(
    rest: &str,
    flex: &mut RecipeFlexibleRuleSpec,
) -> Result<(), KotobaError> {
    let rest = rest.trim();
    let Some((field, rhs)) = rest.split_once(' ') else {
        return Err(KotobaError::Parse {
            line: flex.source_line,
            message: format!("capture inválido: {}", rest),
        });
    };
    let field = field.trim().to_string();
    let field_is_text = field == "text";
    let field_is_speaker = field == "speaker";
    let rhs = rhs.trim();

    if rhs.starts_with("between ") {
        let values = parse_all_quoted(rhs);
        if values.len() >= 2 {
            flex.capture_between = Some((field.clone(), values[0].clone(), values[1].clone()));
            flex.capture_types.insert(field, "line".into());
        }
    } else if let Some(value) = rhs.strip_prefix("after ") {
        flex.capture_after = Some((field.clone(), parse_recipe_value(value)));
        flex.capture_types.insert(field, "line".into());
    } else if let Some(value) = rhs.strip_prefix("before ") {
        flex.capture_before = Some((field.clone(), parse_recipe_value(value)));
        flex.capture_types.insert(field, "line".into());
    } else if let Some(value) = rhs
        .strip_prefix("as field ")
        .or_else(|| rhs.strip_prefix("as column "))
    {
        let (idx_text, sep) = if let Some((left, right)) = value.split_once(" separated by ") {
            (left.trim(), right.trim())
        } else {
            (value.trim(), "tab")
        };
        let index = idx_text.parse::<usize>().map_err(|_| KotobaError::Parse {
            line: flex.source_line,
            message: format!("field inválido em capture: {}", rest),
        })?;
        flex.field_captures.push(RecipeFieldCapture {
            name: field.clone(),
            index,
            separator: parse_recipe_separator(sep),
        });
        flex.capture_types.insert(field, "line".into());
    } else if let Some(value) = rhs.strip_prefix("from attribute ") {
        flex.capture_attribute = Some((field.clone(), parse_recipe_value(value)));
        flex.capture_types.insert(field, "quoted".into());
    } else if let Some(value) = rhs.strip_prefix("as ") {
        let typ = recipe_text_type(value);
        flex.capture_as.insert(field.clone(), typ.clone());
        flex.capture_types.insert(field, typ);
    } else {
        return Err(KotobaError::Parse {
            line: flex.source_line,
            message: format!("capture inválido: {}", rest),
        });
    }
    if field_is_text {
        flex.text_field.get_or_insert("text".into());
    }
    if field_is_speaker {
        flex.speaker_field.get_or_insert("speaker".into());
    }
    Ok(())
}

fn parse_recipe_separator(value: &str) -> String {
    let value = parse_recipe_value(value);
    match value.to_ascii_lowercase().as_str() {
        "tab" | "tabs" | "\\t" => "\t".into(),
        "space" | "spaces" => " ".into(),
        other => other.to_string(),
    }
}

fn compile_recipe_flexible_rule(flex: RecipeFlexibleRuleSpec) -> Result<KotobaRule, KotobaError> {
    let pattern = if let Some(regex) = &flex.raw_regex {
        format!("regex:{}", regex)
    } else if let Some(template) = &flex.template_pattern {
        template.clone()
    } else if !flex.field_captures.is_empty() {
        format!("regex:{}", build_field_capture_regex(&flex))
    } else if flex.capture_between.is_some()
        || flex.capture_after.is_some()
        || flex.capture_before.is_some()
        || flex.capture_attribute.is_some()
    {
        format!("regex:{}", build_structural_capture_regex(&flex))
    } else if !flex.capture_as.is_empty() {
        build_capture_as_pattern(&flex)
    } else if let Some(prefix) = &flex.line_prefix {
        format!("regex:{}(?P<text>.*)", regex::escape(prefix))
    } else {
        "<text:rest>".into()
    };

    let mut extra_fields = BTreeMap::new();
    for (field, typ) in &flex.capture_types {
        extra_fields.insert(format!("__type:{}", field), typ.clone());
    }

    let text_field = flex.text_field.clone().or_else(|| {
        if flex.capture_as.contains_key("text")
            || pattern.contains("?P<text>")
            || pattern.contains("<text:")
        {
            Some("text".into())
        } else {
            None
        }
    });
    let entry_type = flex.entry_type.clone();
    let skip = flex.skip || entry_type.is_none();

    let speaker_field = flex.speaker_field.or_else(|| {
        if pattern.contains("?P<speaker>") || pattern.contains("<speaker:") {
            Some("speaker".into())
        } else {
            None
        }
    });

    Ok(KotobaRule {
        name: flex.name,
        kind: if skip {
            "skip".into()
        } else {
            sanitize_id(entry_type.as_deref().unwrap_or("Narration"))
        },
        source_line: Some(flex.source_line),
        pattern,
        entry_type,
        text_field,
        speaker_field,
        context_field: flex.context_field,
        extra_fields,
        patch_field: flex.patch_field.or_else(|| Some("text".into())),
        speaker_patch_field: flex.speaker_patch_field,
        skip,
        remember: flex.remember,
        forget: flex.forget,
        when: flex.when,
        set: Vec::new(),
        next_pattern: flex.next_pattern,
    })
}

fn build_capture_as_pattern(flex: &RecipeFlexibleRuleSpec) -> String {
    if let Some(typ) = flex.capture_as.get("text") {
        return format!("<text:{}>", typ);
    }
    if let Some((field, typ)) = flex.capture_as.iter().next() {
        return format!("<{}:{}>", field, typ);
    }
    "<text:rest>".into()
}

fn build_structural_capture_regex(flex: &RecipeFlexibleRuleSpec) -> String {
    if let (Some((between_field, open, close)), Some((after_field, after))) =
        (&flex.capture_between, &flex.capture_after)
    {
        if close == after {
            let prefix = structural_capture_prefix(flex.line_prefix.as_deref(), open);
            return format!(
                r#"{}{}(?P<{}>.*?){}(?P<{}>.*)"#,
                prefix,
                regex::escape(open),
                between_field,
                regex::escape(close),
                after_field
            );
        }
    }
    if let (Some((before_field, before)), Some((after_field, after))) =
        (&flex.capture_before, &flex.capture_after)
    {
        if before == after {
            return format!(
                r#"\s*(?P<{}>.*?){}\s*(?P<{}>.*)\s*"#,
                before_field,
                regex::escape(before),
                after_field
            );
        }
    }
    if let Some((field, attr)) = &flex.capture_attribute {
        let prefix = flex
            .line_prefix
            .as_deref()
            .map(regex::escape)
            .unwrap_or_else(|| ".*?".into());
        return format!(
            r#"{}.*?\b{}\s*=\s*(?P<{}>"(?:\\.|[^"])*"|'(?:\\.|[^'])*'|[^\s\]]+).*"#,
            prefix,
            regex::escape(attr),
            field
        );
    }
    if let Some((field, open, close)) = &flex.capture_between {
        return format!(
            r#"\s*{}(?P<{}>.*?){}\s*"#,
            regex::escape(open),
            field,
            regex::escape(close)
        );
    }
    if let Some((field, after)) = &flex.capture_after {
        let prefix = flex
            .line_prefix
            .as_deref()
            .map(regex::escape)
            .unwrap_or_default();
        return format!(
            r#"{}.*?{}(?P<{}>.*)\s*"#,
            prefix,
            regex::escape(after),
            field
        );
    }
    if let Some((field, before)) = &flex.capture_before {
        return format!(r#"\s*(?P<{}>.*?){}.*"#, field, regex::escape(before));
    }
    ".*".into()
}

fn structural_capture_prefix(line_prefix: Option<&str>, opening_delimiter: &str) -> String {
    match line_prefix {
        None | Some("") => r"\s*".into(),
        Some(prefix) if prefix == opening_delimiter => String::new(),
        Some(prefix) => format!("{}.*?", regex::escape(prefix)),
    }
}

fn build_field_capture_regex(flex: &RecipeFlexibleRuleSpec) -> String {
    let sep = flex
        .field_captures
        .first()
        .map(|c| c.separator.clone())
        .unwrap_or_else(|| "\t".into());
    let sep_re = regex::escape(&sep);
    let max_idx = flex
        .field_captures
        .iter()
        .map(|c| c.index)
        .max()
        .unwrap_or(1);
    let mut by_index: BTreeMap<usize, String> = BTreeMap::new();
    for cap in &flex.field_captures {
        by_index.insert(cap.index, cap.name.clone());
    }
    let mut out = String::new();
    if let Some(prefix) = &flex.line_prefix {
        out.push_str(&regex::escape(prefix));
    }
    for idx in 1..=max_idx {
        if idx > 1 {
            out.push_str(&sep_re);
        }
        if let Some(name) = by_index.get(&idx) {
            out.push_str(&format!("(?P<{}>[^{}]*)", name, sep_re));
        } else {
            out.push_str(&format!("[^{}]*", sep_re));
        }
    }
    out.push_str(".*");
    out
}

fn recipe_segment_fields(
    segments: &RecipeSegmentsSpec,
    requested_source: Option<&str>,
) -> Vec<String> {
    let mut fields = segments.fields.clone();
    if fields.is_empty() {
        if let Some(source) = requested_source {
            fields.push(sanitize_id(source));
        } else {
            fields.push(segments.source_field());
        }
    }
    fields
}

fn recipe_effective_segment_field(
    segments: &RecipeSegmentsSpec,
    requested: Option<&str>,
) -> String {
    requested
        .map(sanitize_id)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| segments.source_field())
}

fn recipe_effective_segment_patch_field(segments: &RecipeSegmentsSpec, requested: &str) -> String {
    let requested = requested.trim();
    if requested.is_empty()
        || requested == "text"
        || requested == "content"
        || requested == "segment"
        || requested == "field"
    {
        segments.patch_field()
    } else {
        sanitize_id(requested)
    }
}

fn recipe_segmented_pattern(
    separator: &str,
    fields: &[String],
    source_field: &str,
    source_body: &str,
    other_prefix: Option<&str>,
) -> String {
    let mut out = String::new();
    for field in fields {
        out.push_str(separator);
        if field == source_field {
            out.push_str(source_body);
        } else if let Some(prefix) = other_prefix {
            if prefix.trim().is_empty() {
                out.push_str(&format!("<{}:cell>", field));
            } else {
                out.push_str(&format!("{} <{}:cell>", prefix, field));
            }
        } else {
            out.push_str(&format!("<{}:cell>", field));
        }
    }
    out
}

fn compile_recipe_segmented_dialogue_rule(
    line: &RecipeDialogueLineSpec,
    source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &RecipeSegmentsSpec,
) {
    let Some(separator) = segments.text_separator.as_deref() else {
        return;
    };
    let source_field = recipe_effective_segment_field(segments, line.segment.as_deref());
    let fields = recipe_segment_fields(segments, Some(&source_field));
    let patch_field = recipe_effective_segment_patch_field(segments, &line.patch_field);
    let entry_type = if line.entry_type.trim().is_empty() {
        "Dialogue"
    } else {
        &line.entry_type
    };

    let (source_body, speaker_field) = if let Some((open, close)) = &line.speaker_between {
        let mut bridge = line.text_after.clone().unwrap_or_else(|| close.clone());
        if !bridge.starts_with(close) {
            bridge = format!("{}{}", close, bridge);
        }
        (
            format!("{}<speaker:cell>{}<{}:cell>", open, bridge, source_field),
            Some("speaker"),
        )
    } else if let Some(after) = &line.text_after {
        (format!("{}<{}:cell>", after, source_field), None)
    } else {
        (format!("<{}:cell>", source_field), None)
    };

    spec.rules.push(entry_rule(
        "dialogue_segment",
        &recipe_segmented_pattern(separator, &fields, &source_field, &source_body, None),
        entry_type,
        &source_field,
        speaker_field,
        Some("voice"),
        &patch_field,
        line.speaker_patch_field.as_deref(),
        source_line,
    ));
}

fn compile_recipe_segmented_text_rule(
    entry_type: &str,
    requested_segment: Option<&str>,
    patch_field: &str,
    source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &RecipeSegmentsSpec,
) {
    let Some(separator) = segments.text_separator.as_deref() else {
        return;
    };
    let source_field = recipe_effective_segment_field(segments, requested_segment);
    let fields = recipe_segment_fields(segments, Some(&source_field));
    let patch_field = recipe_effective_segment_patch_field(segments, patch_field);
    let source_body = format!("<{}:cell>", source_field);
    spec.rules.push(entry_rule(
        "text_segment",
        &recipe_segmented_pattern(separator, &fields, &source_field, &source_body, None),
        entry_type,
        &source_field,
        None,
        Some("line_number"),
        &patch_field,
        None,
        source_line,
    ));
}

fn compile_recipe_segmented_choice_rule(
    command: &str,
    requested_segment: Option<&str>,
    patch_field: &str,
    source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &RecipeSegmentsSpec,
) {
    let Some(separator) = segments.choice_separator.as_deref() else {
        return;
    };
    let source_field = recipe_effective_segment_field(segments, requested_segment);
    let fields = recipe_segment_fields(segments, Some(&source_field));
    let patch_field = recipe_effective_segment_patch_field(segments, patch_field);
    let command = command.trim();
    let source_body = if command.is_empty() {
        format!("<{}:cell>", source_field)
    } else {
        format!("{} <{}:cell>", command, source_field)
    };
    let other_prefix = if command.is_empty() {
        None
    } else {
        Some(command)
    };
    spec.rules.push(entry_rule(
        "choice_segment",
        &recipe_segmented_pattern(
            separator,
            &fields,
            &source_field,
            &source_body,
            other_prefix,
        ),
        "ChoiceGroup",
        &source_field,
        None,
        Some("select"),
        &patch_field,
        None,
        source_line,
    ));
}

fn parse_recipe_json(
    name: &str,
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
) -> Result<(), KotobaError> {
    let mut json = KotobaJsonPathSpec {
        name: name.to_string(),
        entries: String::new(),
        text: String::new(),
        speaker: None,
        context: None,
        id: None,
    };
    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if let Some(value) = line.strip_prefix("entries ") {
            json.entries = parse_recipe_value(value);
        } else if let Some(value) = line.strip_prefix("text ") {
            json.text = parse_recipe_value(value);
        } else if let Some(value) = line.strip_prefix("speaker ") {
            json.speaker = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("context ") {
            json.context = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("id ") {
            json.id = Some(parse_recipe_value(value));
        }
    }
    if json.entries.trim().is_empty() || json.text.trim().is_empty() {
        return Err(KotobaError::Parse {
            line: source_line,
            message: format!("json {} exige `entries` e `text`", name),
        });
    }
    spec.json_paths.push(json);
    Ok(())
}

fn parse_recipe_choice(
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &RecipeSegmentsSpec,
) {
    let mut command = String::new();
    let mut patch_field = "text".to_string();
    let mut segment: Option<String> = None;
    let mut split_group = false;
    let mut saved_type: Option<String> = None;
    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if let Some(value) = line.strip_prefix("when starts with ") {
            command = parse_recipe_value(value);
        } else if let Some(value) = line.strip_prefix("field ") {
            segment = Some(sanitize_id(&parse_recipe_value(value)));
        } else if line == "capture choices as quoted" {
            split_group = true;
        } else if let Some(value) = line.strip_prefix("capture text as ") {
            split_group = parse_recipe_value(value) == "quoted";
        } else if let Some(value) = line.strip_prefix("save as ") {
            saved_type = Some(parse_recipe_value(value));
        } else if let Some(value) = line.strip_prefix("patch ") {
            patch_field = parse_recipe_value(value);
        }
    }

    if segment.is_some() && segments.has_choice_segments() {
        compile_recipe_segmented_choice_rule(
            &command,
            segment.as_deref(),
            &patch_field,
            source_line,
            spec,
            segments,
        );
        return;
    }

    if command.is_empty() {
        return;
    }
    let field = if patch_field == "choices" || split_group {
        "choices"
    } else {
        "text"
    };
    let entry_type = saved_type.unwrap_or_else(|| {
        if split_group || field == "choices" {
            "ChoiceGroup".into()
        } else {
            "Choice".into()
        }
    });
    spec.rules.push(entry_rule(
        "choice",
        &format!("{} <{}:rest>", command, field),
        &entry_type,
        field,
        None,
        Some("select"),
        field,
        None,
        source_line,
    ));
}

fn parse_recipe_text(
    children: &[RecipeLine],
    source_line: usize,
    spec: &mut KotobaParserSpec,
    segments: &RecipeSegmentsSpec,
) {
    let mut entry_type = "Narration".to_string();
    let mut section: Option<String> = None;
    let mut text_between: Option<(String, String)> = None;
    let mut text_mode = "content".to_string();
    let mut patch_field = "text".to_string();
    let mut segment: Option<String> = None;
    for child in children {
        let line = child.text.trim().trim_end_matches(';').trim();
        if let Some(value) = line.strip_prefix("save as ") {
            entry_type = parse_recipe_value(value);
        } else if let Some(value) = line.strip_prefix("section ") {
            section = Some(parse_recipe_value(value));
        } else if let Some(value) = line
            .strip_prefix("segment ")
            .or_else(|| line.strip_prefix("language "))
            .or_else(|| line.strip_prefix("field "))
        {
            segment = Some(sanitize_id(&parse_recipe_value(value)));
        } else if line.starts_with("capture text between ") {
            let values = parse_all_quoted(line);
            if values.len() >= 2 {
                text_between = Some((values[0].clone(), values[1].clone()));
            }
        } else if let Some(value) = line.strip_prefix("capture text as ") {
            text_mode = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("patch ") {
            patch_field = recipe_patch_field(value);
        }
    }
    if segment.is_some() && segments.has_text_segments() {
        compile_recipe_segmented_text_rule(
            &entry_type,
            segment.as_deref(),
            &patch_field,
            source_line,
            spec,
            segments,
        );
        return;
    }

    if let Some(section) = section {
        let pattern = if let Some((open, close)) = &text_between {
            recipe_section_text_between_regex(&section, open, close)
        } else if text_mode.contains("quoted") {
            recipe_section_quoted_regex(&section)
        } else {
            format!(
                r#"regex:{}.*?(?P<text>[^\n\r]+).*)"#,
                recipe_section_prefix(&section)
            )
        };
        let mut rule = entry_rule(
            &format!(
                "{}_section_{}",
                kind_to_snake(&entry_type),
                sanitize_id(&section)
            ),
            &pattern,
            &entry_type,
            "text",
            None,
            Some("line_number"),
            &patch_field,
            None,
            source_line,
        );
        rule.extra_fields.insert(
            "__type:text".into(),
            if text_between.is_some() {
                "line".into()
            } else {
                "quoted".into()
            },
        );
        spec.rules.push(rule);
        return;
    }
    spec.rules.push(entry_rule(
        "text",
        "<text:rest>",
        &entry_type,
        "text",
        None,
        Some("line_number"),
        &patch_field,
        None,
        source_line,
    ));
}

fn parse_recipe_numbered_lines(children: &[RecipeLine], spec: &mut KotobaParserSpec) {
    let mut open = String::new();
    let mut close = String::new();
    for child in children {
        let line = child.text.trim();
        if line.starts_with("id between ") {
            let values = parse_all_quoted(line);
            if values.len() >= 2 {
                open = values[0].clone();
                close = values[1].clone();
            }
        }
    }
    if !open.is_empty() && !close.is_empty() {
        spec.line_indexed = Some(IndexedLineSpec { open, close });
    }
}

fn default_recipe_binary_block() -> KotobaBinaryBlockSpec {
    KotobaBinaryBlockSpec {
        name: "binary_record".into(),
        magic: Vec::new(),
        length: default_binary_length(),
        encoding: default_binary_encoding(),
        min_len: default_binary_min_len(),
        profile: "plain".into(),
    }
}

fn is_recipe_binary_line(line: &str) -> bool {
    let line = line.trim().to_ascii_lowercase();
    line.starts_with("magic ")
        || line.starts_with("marker bytes ")
        || line.starts_with("length ")
        || line.starts_with("length is ")
        || line.starts_with("text encoding ")
        || line.starts_with("content encoding ")
        || line.starts_with("encoding ")
        || line.starts_with("text ")
        || line.starts_with("min length ")
        || line.starts_with("min_len ")
        || line.starts_with("min len ")
        || line.starts_with("profile ")
        || line == "patch string"
        || line == "patch record"
        || line == "rebuild length"
}

fn apply_recipe_binary_line(line: &str, block: &mut KotobaBinaryBlockSpec) {
    let line = line.trim();
    if let Some(value) = line.strip_prefix("magic ") {
        block.magic = parse_recipe_bytes(value);
    } else if let Some(value) = line.strip_prefix("marker bytes ") {
        block.magic = parse_recipe_bytes(value);
    } else if let Some(value) = line.strip_prefix("length is ") {
        block.length = parse_recipe_value(value);
    } else if let Some(value) = line.strip_prefix("length ") {
        block.length = parse_recipe_value(value);
    } else if let Some(value) = line.strip_prefix("text encoding ") {
        block.encoding = parse_recipe_value(value);
    } else if let Some(value) = line.strip_prefix("content encoding ") {
        block.encoding = parse_recipe_value(value);
    } else if let Some(value) = line.strip_prefix("encoding ") {
        block.encoding = parse_recipe_value(value);
    } else if let Some(value) = line.strip_prefix("text ") {
        block.encoding = parse_recipe_value(value);
    } else if let Some(value) = line.strip_prefix("min length ") {
        block.min_len = parse_recipe_value(value)
            .parse::<usize>()
            .unwrap_or(default_binary_min_len());
    } else if let Some(value) = line.strip_prefix("min_len ") {
        block.min_len = parse_recipe_value(value)
            .parse::<usize>()
            .unwrap_or(default_binary_min_len());
    } else if let Some(value) = line.strip_prefix("min len ") {
        block.min_len = parse_recipe_value(value)
            .parse::<usize>()
            .unwrap_or(default_binary_min_len());
    } else if let Some(value) = line.strip_prefix("profile ") {
        block.profile = parse_recipe_value(value);
    } else if line == "patch string" || line == "patch record" || line == "rebuild length" { /* accepted; binary rebuild uses payload record offsets and length u32le */
    }
}

fn parse_recipe_bytes(value: &str) -> Vec<u8> {
    if let Some(quoted) = parse_first_quoted(value) {
        if quoted
            .chars()
            .all(|c| c.is_ascii_hexdigit() || c.is_whitespace())
        {
            return parse_hex_bytes(&quoted);
        }
        return quoted.into_bytes();
    }
    parse_hex_bytes(value)
}

fn entry_rule(
    name: &str,
    pattern: &str,
    entry_type: &str,
    text_field: &str,
    speaker_field: Option<&str>,
    context_field: Option<&str>,
    patch_field: &str,
    speaker_patch_field: Option<&str>,
    source_line: usize,
) -> KotobaRule {
    KotobaRule {
        name: name.to_string(),
        kind: sanitize_id(entry_type),
        source_line: Some(source_line),
        pattern: pattern.to_string(),
        entry_type: Some(entry_type.to_string()),
        text_field: Some(text_field.to_string()),
        speaker_field: speaker_field.map(str::to_string),
        context_field: context_field.map(str::to_string),
        patch_field: Some(patch_field.to_string()),
        speaker_patch_field: speaker_patch_field.map(str::to_string),
        ..KotobaRule::default()
    }
}

fn skip_rule(name: &str, pattern: &str, source_line: Option<usize>) -> KotobaRule {
    KotobaRule {
        name: name.to_string(),
        kind: "skip".into(),
        source_line,
        pattern: pattern.to_string(),
        skip: true,
        ..KotobaRule::default()
    }
}

fn recipe_like_to_regex(format: &str) -> String {
    let mut out = String::from("^");
    let chars: Vec<char> = format.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '{' {
            if let Some(end) = chars[i + 1..].iter().position(|c| *c == '}') {
                let name = chars[i + 1..i + 1 + end].iter().collect::<String>();
                if is_valid_regex_capture_name(&name) {
                    out.push_str(&format!(r"(?P<{}>[A-Za-z0-9_]+)", name));
                } else {
                    out.push_str(r"[A-Za-z0-9_]+");
                }
                i += end + 2;
                continue;
            }
        }
        match chars[i] {
            // AAA means exactly three letters. This keeps the original Ef-friendly shorthand.
            'A' => {
                let start = i;
                while i < chars.len() && chars[i] == 'A' {
                    i += 1;
                }
                out.push_str(&format!(r"[A-Za-z]{{{}}}", i - start));
                continue;
            }
            // 0000 means exactly four digits.
            '0' => {
                let start = i;
                while i < chars.len() && chars[i] == '0' {
                    i += 1;
                }
                out.push_str(&format!(r"[0-9]{{{}}}", i - start));
                continue;
            }
            // # means one or more digits. Useful for bgm#, se#, map#, etc.
            '#' => {
                out.push_str(r"[0-9]+");
                i += 1;
                continue;
            }
            // * means a filename/id tail: letters, numbers or underscore, zero or more.
            '*' => {
                out.push_str(r"[A-Za-z0-9_]*");
                i += 1;
                continue;
            }
            // ? means a single filename/id character.
            '?' => {
                out.push_str(r"[A-Za-z0-9_]");
                i += 1;
                continue;
            }
            ch => {
                out.push_str(&regex::escape(&ch.to_string()));
                i += 1;
            }
        }
    }
    out.push('$');
    out
}

fn is_valid_regex_capture_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn escape_recipe_quote(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn parse_simple_value(value: &str) -> String {
    parse_first_quoted(value)
        .unwrap_or_else(|| value.trim().trim_end_matches(';').trim().to_string())
}

fn parse_simple_values(value: &str) -> Vec<String> {
    let quoted = parse_all_quoted(value);
    if !quoted.is_empty() {
        return quoted;
    }
    value
        .split(',')
        .flat_map(|part| part.split_whitespace())
        .map(|part| part.trim().trim_end_matches(';').to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

fn parse_simple_protect_items(value: &str) -> Vec<KotobaProtectRule> {
    let line = value.trim().trim_end_matches(';').trim();
    if line.is_empty() {
        return Vec::new();
    }
    if let Some(value) = line.strip_prefix("pattern ") {
        return vec![KotobaProtectRule::Pattern(parse_simple_value(value))];
    }
    if let Some(value) = line.strip_prefix("literal ") {
        return vec![KotobaProtectRule::Literal(parse_simple_value(value))];
    }
    if let Some(value) = line.strip_prefix("bracket ") {
        let values = parse_simple_values(value);
        if values.len() >= 2 {
            return vec![KotobaProtectRule::Pattern(format!(
                "{}[^{}]+{}",
                regex::escape(&values[0]),
                regex::escape(&values[1]),
                regex::escape(&values[1])
            ))];
        }
    }
    if let Some(value) = line.strip_prefix("tag ") {
        let values = parse_simple_values(value);
        if values.len() >= 2 {
            return vec![KotobaProtectRule::Pattern(format!(
                "{}[^{}]+{}",
                regex::escape(&values[0]),
                regex::escape(&values[1]),
                regex::escape(&values[1])
            ))];
        }
    }
    if let Some(value) = line.strip_prefix("quote ") {
        let values = parse_simple_values(value);
        if values.len() >= 2 {
            return vec![KotobaProtectRule::Pattern(format!(
                "{}.*?{}",
                regex::escape(&values[0]),
                regex::escape(&values[1])
            ))];
        }
    }
    if line == "slash_commands" || line == "slash command" || line == "slash-commands" {
        return vec![KotobaProtectRule::Pattern(r"\\[A-Za-z]+".into())];
    }
    if line == "hash_numbers"
        || line == "hash numbers"
        || line == "hash-number"
        || line == "hash-numbers"
    {
        return vec![KotobaProtectRule::Pattern(r"#[0-9]+".into())];
    }
    if line == "html_tags" || line == "html tags" || line == "angle_tags" || line == "angle tags" {
        return vec![KotobaProtectRule::Pattern(r"<[^>]+>".into())];
    }
    if line == "ruby_tags" || line == "ruby tags" {
        return vec![KotobaProtectRule::Pattern(r"</?RUBY[^>]*>".into())];
    }
    if line == "newlines" || line == "linebreaks" || line == "line breaks" {
        return vec![KotobaProtectRule::Pattern(r"\r?\n".into())];
    }
    if line == "kag_tags" || line == "kag tags" || line == "brackets" {
        return vec![KotobaProtectRule::Pattern(r"\[[^\]]+\]".into())];
    }
    vec![KotobaProtectRule::Literal(parse_simple_value(line))]
}

fn match_rule(
    rule: &KotobaRule,
    line: &str,
    spec: &KotobaParserSpec,
) -> Result<Option<BTreeMap<String, String>>, KotobaError> {
    if rule.pattern.trim().is_empty() {
        return Ok(None);
    }
    if let Some(regex_src) = rule.pattern.trim().strip_prefix("regex:") {
        let re = Regex::new(&anchor_regex(regex_src)).map_err(|e| KotobaError::Regex {
            rule: rule.name.clone(),
            message: e.to_string(),
        })?;
        let Some(caps) = re.captures(line) else {
            return Ok(None);
        };
        return captures_from_direct_regex(&re, &caps, rule, spec);
    }
    let (regex_src, capture_types) = compile_pattern(&rule.pattern);
    let re = Regex::new(&format!("^{}$", regex_src)).map_err(|e| KotobaError::Regex {
        rule: rule.name.clone(),
        message: e.to_string(),
    })?;
    let Some(caps) = re.captures(line) else {
        return Ok(None);
    };
    let mut out = BTreeMap::new();
    for (field, typ) in capture_types {
        let Some(m) = caps.name(&field) else {
            continue;
        };
        let value = m.as_str().to_string();
        if !validate_capture(&value, &typ, spec) {
            return Ok(None);
        }
        out.insert(field, value);
    }
    Ok(Some(out))
}

fn anchor_regex(regex_src: &str) -> String {
    let trimmed = regex_src.trim();
    let mut out = String::new();
    if !trimmed.starts_with('^') {
        out.push('^');
    }
    out.push_str(trimmed);
    if !trimmed.ends_with('$') {
        out.push('$');
    }
    out
}

fn regex_capture_type<'a>(rule: &'a KotobaRule, field: &str) -> Option<&'a str> {
    rule.extra_fields
        .get(&format!("__type:{}", field))
        .map(|v| v.as_str())
}

fn captures_from_direct_regex(
    re: &Regex,
    caps: &regex::Captures<'_>,
    rule: &KotobaRule,
    spec: &KotobaParserSpec,
) -> Result<Option<BTreeMap<String, String>>, KotobaError> {
    let mut out = BTreeMap::new();
    for name in re.capture_names().flatten() {
        let Some(m) = caps.name(name) else {
            continue;
        };
        let value = m.as_str().to_string();
        if let Some(typ) = regex_capture_type(rule, name) {
            if !validate_capture(&value, typ, spec) {
                return Ok(None);
            }
        }
        out.insert(name.to_string(), value);
    }
    Ok(Some(out))
}

fn compile_pattern(pattern: &str) -> (String, BTreeMap<String, String>) {
    let mut out = String::new();
    let mut capture_types = BTreeMap::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0usize;
    let mut literal = String::new();
    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(end) = chars[i + 1..].iter().position(|c| *c == '>') {
                flush_literal(&mut out, &literal);
                literal.clear();
                let spec = chars[i + 1..i + 1 + end].iter().collect::<String>();
                if let Some((field, typ)) = spec.split_once(':') {
                    let field = field.trim().to_string();
                    let typ = typ.trim().to_string();
                    out.push_str(&capture_regex(&field, &typ));
                    capture_types.insert(field, typ);
                } else {
                    out.push_str(&regex::escape(&format!("<{}>", spec)));
                }
                i += end + 2;
                continue;
            }
        }
        literal.push(chars[i]);
        i += 1;
    }
    flush_literal(&mut out, &literal);
    (out, capture_types)
}

fn capture_regex(field: &str, typ: &str) -> String {
    match typ {
        "number" => format!(r"(?P<{}>\d+)", field),
        "word" | "name" => format!(r"(?P<{}>\S+)", field),
        "quoted" => format!(
            r#"(?P<{}>"(?:\\.|[^"])*"|“[^”]*”|‘[^’]*’|≪[^≫]*≫|「[^」]*」|挌.*?拮|抛.*?拉|Åg.*?Åh|�g.*?�h)"#,
            field
        ),
        "rest" | "line" => format!(r"(?P<{}>.*)", field),
        _ => format!(r"(?P<{}>.*?)", field),
    }
}

fn flush_literal(out: &mut String, literal: &str) {
    let mut buf = String::new();
    let mut in_space = false;
    for ch in literal.chars() {
        if ch.is_whitespace() {
            if !buf.is_empty() {
                out.push_str(&regex::escape(&buf));
                buf.clear();
            }
            if !in_space {
                out.push_str(r"\s+");
                in_space = true;
            }
        } else {
            in_space = false;
            buf.push(ch);
        }
    }
    if !buf.is_empty() {
        out.push_str(&regex::escape(&buf));
    }
}

fn validate_capture(value: &str, typ: &str, spec: &KotobaParserSpec) -> bool {
    match typ {
        "number" => value.chars().all(|c| c.is_ascii_digit()),
        "word" | "name" => !value.trim().is_empty() && !value.chars().any(char::is_whitespace),
        "quoted" | "rest" | "line" | "cell" => true,
        other => {
            let Some(t) = spec.types.get(other) else {
                return true;
            };
            let candidate = if t.trim { value.trim() } else { value };
            if !t.values.is_empty() && !t.values.iter().any(|v| v == candidate) {
                return false;
            }
            if !t.patterns.is_empty() {
                let mut matched_any_pattern = false;
                for pattern in &t.patterns {
                    if let Ok(re) = Regex::new(pattern) {
                        if re.is_match(candidate) {
                            matched_any_pattern = true;
                            break;
                        }
                    }
                }
                if !matched_any_pattern {
                    return false;
                }
            }
            true
        }
    }
}

fn normalized_fields(
    captures: &BTreeMap<String, String>,
    spec: &KotobaParserSpec,
) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for (k, v) in captures {
        fields.insert(k.clone(), normalize_captured(v, "line", spec));
    }
    fields
}

fn normalize_captured(value: &str, typ: &str, spec: &KotobaParserSpec) -> String {
    let trimmed = value.trim();
    if typ == "quoted" || is_wrapped_quote(trimmed, spec) {
        strip_quote_pair(trimmed, spec).unwrap_or_else(|| trimmed.to_string())
    } else {
        trimmed.to_string()
    }
}

fn is_wrapped_quote(value: &str, spec: &KotobaParserSpec) -> bool {
    strip_quote_pair(value, spec).is_some()
}

fn strip_quote_pair(value: &str, spec: &KotobaParserSpec) -> Option<String> {
    let defaults = default_quote_pairs();
    for (open, close) in spec.quote_pairs.iter().chain(defaults.iter()) {
        if value.starts_with(open)
            && value.ends_with(close)
            && value.len() >= open.len() + close.len()
        {
            let inner = &value[open.len()..value.len() - close.len()];
            return Some(inner.replace("\\\"", "\""));
        }
    }
    None
}

fn split_choice_cell(value: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Ok(re) = Regex::new(r#"([A-Za-z0-9_]+):"([^"]*)""#) {
        for caps in re.captures_iter(value) {
            out.push((
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2).unwrap().as_str().to_string(),
            ));
        }
    }
    out
}

fn capture_type_for<'a>(rule: &'a KotobaRule, field: &str) -> Option<&'a str> {
    if let Some(typ) = regex_capture_type(rule, field) {
        return Some(typ);
    }
    let mut needle = String::new();
    needle.push('<');
    needle.push_str(field);
    needle.push(':');
    let start = rule.pattern.find(&needle)? + needle.len();
    let rest = &rule.pattern[start..];
    let end = rest.find('>')?;
    Some(&rest[..end])
}

fn global_skip(line: &str, spec: &KotobaParserSpec) -> bool {
    let trimmed_start = line.trim_start();
    let trimmed = line.trim();
    for rule in &spec.skip_rules {
        match rule {
            KotobaSkipRule::Empty if trimmed.is_empty() => return true,
            KotobaSkipRule::Prefix(prefix) | KotobaSkipRule::StartsWith(prefix)
                if trimmed_start.starts_with(prefix) =>
            {
                return true
            }
            KotobaSkipRule::Contains(token) if line.contains(token) => return true,
            KotobaSkipRule::EndsWith(token) if trimmed.ends_with(token) => return true,
            KotobaSkipRule::Equals(token) if trimmed == token => return true,
            KotobaSkipRule::Unless(token) | KotobaSkipRule::UnlessContains(token)
                if !line.contains(token) =>
            {
                return true
            }
            KotobaSkipRule::UnlessStartsWith(prefix) if !trimmed_start.starts_with(prefix) => {
                return true
            }
            KotobaSkipRule::UnlessEndsWith(token) if !trimmed.ends_with(token) => return true,
            KotobaSkipRule::UnlessEquals(token) if trimmed != token => return true,
            KotobaSkipRule::Matching(pattern)
                if Regex::new(pattern)
                    .map(|re| re.is_match(trimmed))
                    .unwrap_or(false) =>
            {
                return true
            }
            KotobaSkipRule::Asset if is_asset_line(line) => return true,
            _ => {}
        }
    }
    false
}

fn is_asset_line(line: &str) -> bool {
    let t = line.trim();
    if t.is_empty() {
        return false;
    }
    let lower = t.to_lowercase();
    lower == "black"
        || lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".webp")
        || lower.ends_with(".ogg")
        || lower.ends_with(".wav")
        || lower.ends_with(".bin")
        || lower.ends_with(".bmp")
        || lower.ends_with(".mp3")
        || Regex::new(r"^(bg|bgm|se|sp|ev|map)[A-Za-z0-9_]*$")
            .map(|re| re.is_match(t))
            .unwrap_or(false)
}

fn strip_index_prefix<'a>(line: &'a str, spec: &IndexedLineSpec) -> Option<(usize, &'a str)> {
    let rest = line.strip_prefix(&spec.open)?;
    let end = rest.find(&spec.close)?;
    let index = rest[..end].parse().ok()?;
    Some((index, &rest[end + spec.close.len()..]))
}

fn replace_once(line: &mut String, from: &str, to: &str) -> bool {
    if from.is_empty() {
        return false;
    }
    if let Some(pos) = line.find(from) {
        let end = pos + from.len();
        line.replace_range(pos..end, to);
        true
    } else {
        false
    }
}

fn split_preserving_newline(source: &str) -> Vec<String> {
    if source.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<String> = source
        .split_inclusive('\n')
        .map(|s| s.to_string())
        .collect();
    if !source.ends_with('\n') && out.is_empty() {
        out.push(source.to_string());
    }
    out
}

fn default_quote_pairs() -> Vec<(String, String)> {
    vec![
        ("\"".into(), "\"".into()),
        ("'".into(), "'".into()),
        ("“".into(), "”".into()),
        ("「".into(), "」".into()),
        ("≪".into(), "≫".into()),
        ("Åg".into(), "Åh".into()),
    ]
}
fn parse_first_quoted(value: &str) -> Option<String> {
    parse_all_quoted(value).into_iter().next()
}

fn parse_all_quoted(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = value.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '"' {
            let start = i + 1;
            i += 1;
            let mut s = String::new();
            while i < chars.len() {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    let escaped = match chars[i + 1] {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        other => other,
                    };
                    s.push(escaped);
                    i += 2;
                } else if chars[i] == '"' {
                    out.push(s);
                    i += 1;
                    break;
                } else {
                    s.push(chars[i]);
                    i += 1;
                }
            }
            if start >= chars.len() {
                break;
            }
        } else {
            i += 1;
        }
    }
    out
}

fn sanitize_id(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() {
        "entry".into()
    } else {
        out
    }
}

fn kind_to_snake(kind: &str) -> String {
    match kind {
        "Dialogue" => "dialogue".into(),
        "Narration" => "narration".into(),
        "Choice" | "ChoiceGroup" => "choice".into(),
        other => sanitize_id(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_recipe_extracts_and_rebuilds_dialogue() {
        let parser = r#"parser Test:
    file ".txt"
    encoding utf8
    dialogue:
        when starts with ".message"
        capture id as number
        capture speaker before quoted text
        capture text as quoted
        save as Dialogue
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let input = ".message 1 Alice \"Hello.\"\n";
        let (entries, report) = extract(input, &spec).unwrap();
        assert_eq!(report.total_entries, 1);
        assert_eq!(entries[0].speaker.as_deref(), Some("Alice"));
        assert_eq!(entries[0].text, "Hello.");
        let rebuilt = rebuild(
            input,
            &spec,
            &[KotobaPatchInput {
                id: entries[0].id.clone(),
                index: 0,
                source: entries[0].text.clone(),
                translation: "Olá.".into(),
                speaker_translation: String::new(),
            }],
        )
        .unwrap();
        assert!(rebuilt.contains("\"Olá.\""));
    }

    #[test]
    fn canonical_dialogue_supports_delimited_speaker_and_text() {
        let parser = r#"parser GoreScreamingShowSystemNNN:
    file ".txt"
    encoding utf8
    dialogue:
        when starts with "<"
        capture speaker between "<" and ">"
        capture text after ">"
        save as Dialogue
        patch speaker
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let input = "<Kyoji>(That's Mt. Nishimori.)\n";
        let (entries, report) = extract(input, &spec).unwrap();
        assert_eq!(report.total_entries, 1);
        assert_eq!(entries[0].speaker.as_deref(), Some("Kyoji"));
        assert_eq!(entries[0].text, "(That's Mt. Nishimori.)");

        let rebuilt = rebuild(
            input,
            &spec,
            &[KotobaPatchInput {
                id: entries[0].id.clone(),
                index: 0,
                source: entries[0].text.clone(),
                translation: "(Aquele é o Monte Nishimori.)".into(),
                speaker_translation: "Kyouji".into(),
            }],
        )
        .unwrap();
        assert_eq!(rebuilt, "<Kyouji>(Aquele é o Monte Nishimori.)\n");
    }

    #[test]
    fn legacy_universal_rule_remains_read_compatible() {
        let parser = r#"parser Rules:
    file ".txt"
    rule Dialogue:
        when format "<speaker:word>: <text:rest>"
        save as Dialogue
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let (entries, _) = extract("Alice: Hello\n", &spec).unwrap();
        assert_eq!(entries[0].speaker.as_deref(), Some("Alice"));
        assert_eq!(entries[0].text, "Hello");
    }

    #[test]
    fn removed_syntaxes_and_aliases_are_hard_errors() {
        let removed = [
            "name: Old\nid: old\n",
            "kotoba Old { target \".txt\"; }",
            "parser Old:\n    target \".txt\"\n",
            "parser Old:\n    message:\n        line starts with \".message\"\n",
            "parser Old:\n    text block:\n        capture text as rest\n",
            "parser Old:\n    indexed lines:\n        id between \"<\" and \">\"\n",
            "parser Old:\n    binary strings:\n        min length 4\n",
            "parser Old:\n    inside strings:\n        patch string\n",
            "parser Old:\n    ignore:\n        lines starting with \";\"\n",
            "parser Old:\n    rule X:\n        take text as rest\n",
            "parser Old:\n    dialogue:\n        patch speaker speaker\n",
            "parser Old:\n    text:\n        unknown command\n",
        ];
        for source in removed {
            assert!(parse_source(source).is_err(), "deveria rejeitar: {source}");
        }
    }

    #[test]
    fn named_text_block_does_not_restore_removed_text_block_alias() {
        assert!(parse_source(
            "parser Named:\n    text SceneTitle:\n        capture text as rest\n"
        )
        .is_ok());
        assert!(
            parse_source("parser Old:\n    text block:\n        capture text as rest\n").is_err()
        );
    }

    #[test]
    fn canonical_json_block_extracts_entries() {
        let parser = r#"parser Json:
    file ".json"
    json Line:
        entries "$.lines[*]"
        text "text"
        speaker "speaker"
"#;
        let spec = parse_source(parser).unwrap();
        let (entries, report) =
            extract(r#"{"lines":[{"speaker":"A","text":"Hi"}]}"#, &spec).unwrap();
        assert_eq!(report.total_entries, 1);
        assert_eq!(entries[0].text, "Hi");
    }

    #[test]
    fn recipe_encoding_controls_output_without_forcing_source_decoding() {
        let parser = r#"parser OutputEncoding:
    file ".txt"
    encoding windows-1252
    text:
        capture text as rest
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let (source_bytes, _, source_errors) = SHIFT_JIS.encode("これは日本語です。\r\n");
        assert!(!source_errors);

        let (entries, extract_report) = extract_bytes(&source_bytes, &spec).unwrap();
        assert_eq!(extract_report.total_entries, 1);
        assert_eq!(entries[0].text, "これは日本語です。");

        let patches = [KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "Olá, ação.".into(),
            speaker_translation: String::new(),
        }];
        let (rebuilt, rebuild_report) =
            rebuild_bytes_with_report(&source_bytes, &spec, &patches).unwrap();
        assert_eq!(rebuild_report.applied_patches, 1);
        assert!(std::str::from_utf8(&rebuilt).is_err());
        let (decoded, _, output_errors) = WINDOWS_1252.decode(&rebuilt);
        assert!(!output_errors);
        assert_eq!(decoded, "Olá, ação.\r\n");
    }

    #[test]
    fn absent_recipe_encoding_preserves_detected_source_encoding() {
        let parser = r#"parser PreserveSource:
    file ".txt"
    text:
        capture text as rest
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let (source_bytes, _, source_errors) = SHIFT_JIS.encode("日本語\r\n");
        assert!(!source_errors);
        let (entries, _) = extract_bytes(&source_bytes, &spec).unwrap();
        let patches = [KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "翻訳".into(),
            speaker_translation: String::new(),
        }];
        let rebuilt = rebuild_bytes(&source_bytes, &spec, &patches).unwrap();
        let (decoded, _, errors) = SHIFT_JIS.decode(&rebuilt);
        assert!(!errors);
        assert_eq!(decoded, "翻訳\r\n");
    }

    #[test]
    fn text_rebuild_preserves_original_bytes_outside_the_patched_capture() {
        let parser = r#"parser MixedBytes:
    file ".txt"
    encoding windows-1252
    dialogue:
        capture text between "「" and "」"
        save as Dialogue
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let (source_bytes, _, source_errors) = SHIFT_JIS.encode("「Original」\r\n");
        assert!(!source_errors);
        let (entries, _) = extract_bytes(&source_bytes, &spec).unwrap();
        assert_eq!(entries.len(), 1);
        let original_start = source_bytes
            .windows(b"Original".len())
            .position(|window| window == b"Original")
            .unwrap();
        let original_end = original_start + b"Original".len();
        let patch = KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "Olá".into(),
            speaker_translation: String::new(),
        };

        let (rebuilt, report) = rebuild_bytes_with_report(&source_bytes, &spec, &[patch]).unwrap();

        assert_eq!(report.applied_patches, 1);
        assert!(report.lossy_replacements.is_empty());
        assert_eq!(&rebuilt[..original_start], &source_bytes[..original_start]);
        assert_eq!(&rebuilt[original_start..original_start + 3], b"Ol\xE1");
        assert_eq!(
            &rebuilt[original_start + 3..],
            &source_bytes[original_end..]
        );
    }

    #[test]
    fn lossy_rebuild_replaces_only_unencodable_translation_characters() {
        let parser = r#"parser LossyMixedBytes:
    file ".txt"
    encoding windows-1252
    dialogue:
        capture text between "「" and "」"
        save as Dialogue
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let (source_bytes, _, source_errors) = SHIFT_JIS.encode("「Original」\r\n");
        assert!(!source_errors);
        let (entries, _) = extract_bytes(&source_bytes, &spec).unwrap();
        let original_start = source_bytes
            .windows(b"Original".len())
            .position(|window| window == b"Original")
            .unwrap();
        let original_end = original_start + b"Original".len();
        let patch = KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "Olá →".into(),
            speaker_translation: String::new(),
        };

        let strict_error =
            rebuild_bytes_with_report(&source_bytes, &spec, &[patch.clone()]).unwrap_err();
        let strict_message = strict_error.to_string();
        assert!(strict_message.contains("U+2192"));
        assert!(strict_message.contains(&entries[0].id));

        let (rebuilt, report) =
            rebuild_bytes_with_report_mode(&source_bytes, &spec, &[patch], true).unwrap();
        assert_eq!(report.lossy_replacements.len(), 1);
        assert_eq!(
            report.lossy_replacements[0].characters[0].codepoint,
            "U+2192"
        );
        assert_eq!(&rebuilt[..original_start], &source_bytes[..original_start]);
        assert_eq!(&rebuilt[original_start..original_start + 5], b"Ol\xE1 ?");
        assert_eq!(
            &rebuilt[original_start + 5..],
            &source_bytes[original_end..]
        );
    }

    #[test]
    fn raw_byte_substitution_is_independent_from_recipe_encoding() {
        let parser = r#"parser RemappedFont:
    file ".txt"
    encoding windows-1252
    text:
        capture text as rest
        patch text
"#;
        let spec = parse_source(parser).unwrap();
        let source_bytes = b"Original\r\n";
        let (entries, _) = extract_bytes(source_bytes, &spec).unwrap();
        let patch = KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "À ação".into(),
            speaker_translation: String::new(),
        };
        let substitutions = vec![
            KotobaCharacterSubstitution {
                source: "À".into(),
                mode: "bytes".into(),
                target: "A6".into(),
            },
            KotobaCharacterSubstitution {
                source: "ã".into(),
                mode: "bytes".into(),
                target: "BB".into(),
            },
        ];

        let (rebuilt, report) =
            rebuild_bytes_with_report_options(source_bytes, &spec, &[patch], false, &substitutions)
                .unwrap();

        assert_eq!(report.applied_patches, 1);
        assert_eq!(rebuilt, b"\xA6 a\xE7\xBBo\r\n");
    }

    #[test]
    fn text_substitution_uses_the_recipe_encoding() {
		let parser = concat!(
			"parser Windows1252TextMap:\n",
			"    file \".txt\"\n",
			"    encoding windows-1252\n",
			"    text:\n",
			"        capture text as rest\n",
			"        patch text\n",
		);

        let spec = parse_source(parser).unwrap();
        let source_bytes = b"Original\n";
        let (entries, _) = extract_bytes(source_bytes, &spec).unwrap();

        let patch = KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "ação".into(),
            speaker_translation: String::new(),
        };

        let substitutions = vec![KotobaCharacterSubstitution {
            source: "ç".into(),
            mode: "text".into(),
            target: "é".into(),
        }];

        let (rebuilt, _) =
            rebuild_bytes_with_report_options(source_bytes, &spec, &[patch], false, &substitutions)
                .unwrap();

        assert_eq!(rebuilt, b"a\xE9\xE3o\n");
    }
}
