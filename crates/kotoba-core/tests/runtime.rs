use kotoba_core::{
    extract, parse_source, preview_rebuild, preview_rebuild_bytes, rebuild, KotobaPatchInput,
};

#[test]
fn ef_message_extract_and_rebuild_fixture() {
    let parser = include_str!("../../../tests/fixtures/parsers/ef_message.kotoba");
    let input = include_str!("../../../tests/fixtures/ef/sample.sc");
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 2);
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
fn multilingual_bs5_canonical_parser_compiles() {
    let parser = include_str!("../../../tests/fixtures/parsers/multilingual_bs5.kotoba");
    let spec = parse_source(parser).unwrap();
    assert_eq!(spec.name, "MultilingualBS5");
    assert!(spec.rules.iter().any(|r| r.name == "dialogue_segment"));
}

#[test]
fn rust_runtime_extracts_real_aokana_excerpt() {
    let parser = include_str!("../../../tests/fixtures/parsers/aokana_bs5.kotoba");
    let input = include_str!("../../../tests/fixtures/real/aokana_ep01_excerpt.bs5");
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 4);
    assert_eq!(entries[0].speaker.as_deref(), Some("Girl"));
    assert_eq!(entries[0].text, "...I need to go now.");
    assert_eq!(entries[3].speaker.as_deref(), Some("Boy"));
    assert!(entries[3].text.contains("No tears today"));
}

#[test]
fn rust_runtime_decodes_cp932_ef_excerpt() {
    let parser = include_str!("../../../examples/ef_command_message.kotoba");
    let input = include_bytes!("../../../tests/fixtures/real/ef_100_01_excerpt.sc");
    let spec = parse_source(parser).unwrap();
    let (entries, report) = kotoba_core::extract_bytes(input, &spec).unwrap();
    assert!(report.total_entries >= 4);
    assert_eq!(entries[0].speaker.as_deref(), Some("Yuuko"));
    assert_eq!(entries[0].text, "Oh my.");
}

#[test]
fn rust_runtime_extracts_json_array_strings() {
    let parser = include_str!("../../../tests/fixtures/parsers/json_array_strings.kotoba");
    let input = include_str!("../../../tests/fixtures/real/array_strings_excerpt.json");
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 3);
    assert_eq!(entries[0].speaker.as_deref(), Some("Keisuke"));
    assert_eq!(entries[0].text, "...");
    assert_eq!(entries[2].kind, "narration");
}

#[test]
fn rust_runtime_decodes_utf16_plaintext_script() {
    let parser = include_str!("../../../tests/fixtures/parsers/majikoi_txt.kotoba");
    let input = include_bytes!("../../../tests/fixtures/real/majikoi_act_b_excerpt.txt");
    let spec = parse_source(parser).unwrap();
    let (entries, report) = kotoba_core::extract_bytes(input, &spec).unwrap();
    assert!(report.total_entries >= 8);
    assert!(
        entries
            .iter()
            .any(|entry| entry.speaker.as_deref() == Some("Momoyo")
                && entry.text.contains("lil' bro"))
    );
}

#[test]
fn rust_runtime_extracts_binary_ascii_text_candidates() {
    let input = include_bytes!("../../../tests/fixtures/real/nut_binary_excerpt.bin");
    let entries = kotoba_core::extract_binary_text_entries(input, 4);
    assert!(entries
        .iter()
        .any(|entry| entry.text.contains("performance")));
    assert!(entries.iter().any(|entry| entry.text.contains("kindness")));
    assert!(entries.iter().all(|entry| entry
        .context
        .as_deref()
        .unwrap_or_default()
        .starts_with("offset:")));
}

#[test]
fn preview_rebuild_reports_line_diff_without_rebuild_output() {
    let parser = include_str!("../../../tests/fixtures/parsers/ef_message.kotoba");
    let input = include_str!("../../../tests/fixtures/ef/sample.sc");
    let spec = parse_source(parser).unwrap();
    let (entries, _) = extract(input, &spec).unwrap();
    let preview = preview_rebuild(
        &input,
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

    assert!(preview.changed);
    assert_eq!(preview.report.total_patches, 1);
    assert_eq!(preview.report.applied_patches, 1);
    assert_eq!(preview.changes.len(), 1);
    assert_eq!(preview.changes[0].line, 1);
    assert!(preview.changes[0].before.contains("Hello."));
    assert!(preview.changes[0].after.contains("Olá."));
    assert_eq!(preview.changes[0].entries[0].id, entries[0].id);
}

#[test]
fn preview_rebuild_reports_no_change_for_empty_patch() {
    let parser = include_str!("../../../tests/fixtures/parsers/ef_message.kotoba");
    let input = include_str!("../../../tests/fixtures/ef/sample.sc");
    let spec = parse_source(parser).unwrap();
    let (entries, _) = extract(input, &spec).unwrap();
    let preview = preview_rebuild(
        &input,
        &spec,
        &[KotobaPatchInput {
            id: entries[0].id.clone(),
            index: 0,
            source: entries[0].text.clone(),
            translation: "".into(),
            speaker_translation: String::new(),
        }],
    )
    .unwrap();

    assert!(!preview.changed);
    assert_eq!(preview.report.total_patches, 1);
    assert_eq!(preview.report.applied_patches, 0);
    assert_eq!(preview.report.skipped_patches, 1);
    assert!(preview.changes.is_empty());
}

#[test]
fn preview_rebuild_bytes_decodes_with_parser_encoding() {
    let parser = include_str!("../../../examples/ef_command_message.kotoba");
    let input = include_bytes!("../../../tests/fixtures/real/ef_100_01_excerpt.sc");
    let spec = parse_source(parser).unwrap();
    let (entries, _) = kotoba_core::extract_bytes(input, &spec).unwrap();
    let preview = preview_rebuild_bytes(
        input,
        &spec,
        &[KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "Ora.".into(),
            speaker_translation: String::new(),
        }],
    )
    .unwrap();

    assert!(preview.changed);
    assert_eq!(preview.report.applied_patches, 1);
    assert!(preview.changes[0].after.contains("Ora."));
}

#[test]
fn rust_runtime_extracts_gls_nut_length_prefixed_blocks() {
    let parser = include_str!("../../../examples/flexible_binary_nut.kotoba");
    let input = include_bytes!("../../../tests/fixtures/real/gls_nut_ma00_excerpt.nut");
    let spec = parse_source(parser).unwrap();
    assert_eq!(spec.binary_blocks.len(), 1);

    let (entries, report) = kotoba_core::extract_bytes(input, &spec).unwrap();
    assert!(report.total_entries >= 3);
    assert!(entries.iter().any(|entry| entry.text.contains("four acts")));

    let voiced = entries
        .iter()
        .find(|entry| entry.context.as_deref() == Some("voice/ma00/0000010e237"))
        .unwrap();
    assert_eq!(voiced.kind, "dialogue");
    assert_eq!(voiced.speaker.as_deref(), Some("ｅｔｃ／部隊長"));
    assert!(voiced.text.contains("Hand him over at once"));
    assert!(voiced.fields.get("raw").unwrap().contains("<voice name='"));
    assert!(voiced.fields.get("payload_offset").is_some());
}

#[test]
fn rust_runtime_summarizes_binary_program_blocks() {
    let parser = include_str!("../../../examples/flexible_binary_nut.kotoba");
    let summary = kotoba_core::summarize_source(parser).unwrap();
    assert_eq!(summary.binary_blocks.len(), 1);
    assert!(summary
        .symbols
        .iter()
        .any(|symbol| symbol.kind == "binary" && symbol.name == "binary_record"));
}

#[test]
fn rust_runtime_exposes_formal_language_spec() {
    let spec = kotoba_core::language_spec();
    assert_eq!(spec.version, "0.3-recipe");
    assert!(spec
        .execution_model
        .iter()
        .any(|feature| feature.name == "recipe" && feature.status == "implemented"));
    assert!(spec
        .rebuild_model
        .iter()
        .any(|feature| feature.name == "field_patch"));
    assert!(spec
        .binary_model
        .iter()
        .any(|feature| feature.name == "binary_rebuild" && feature.status == "implemented"));
    assert!(spec
        .required_runtime_commands
        .iter()
        .any(|command| command == "language-spec"));
}

#[test]
fn rust_runtime_accepts_recipe_ef_parser() {
    let parser = include_str!("../../../examples/ef_recipe.kotoba");
    let input = ".message 1 EFG-Yuuko-0001 Yuuko \"Hello.\"\n.message 2 EFG-Hiro-0002 \"Voice fallback.\"\n.select \"Yes\" \"No\"\n";
    let spec = parse_source(parser).unwrap();
    assert_eq!(spec.name, "EfFairyTaleMusicaSC");
    assert!(spec.types.contains_key("VoiceId"));
    assert!(spec
        .rules
        .iter()
        .any(|r| r.name == "dialogue_line_voice_fallback_quoted"));
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 3);
    assert_eq!(entries[0].speaker.as_deref(), Some("Yuuko"));
    assert_eq!(entries[0].text, "Hello.");
    assert_eq!(entries[1].speaker.as_deref(), Some("Hiro"));
    assert_eq!(entries[1].text, "Voice fallback.");
    assert_eq!(entries[2].kind, "choice");
}

#[test]
fn rust_runtime_accepts_recipe_kirikiri_parser() {
    let parser = include_str!("../../../examples/kirikiri_recipe.kotoba");
    let input = ";comment\n*label\n[cm]\nHello[cr]\n";
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 1);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].text, "Hello[cr]");
}

#[test]
fn rust_runtime_accepts_recipe_friendly_like_filters() {
    let parser = include_str!("../../../examples/numbered_like_recipe.kotoba");
    let input = "<0>Prólogo\n<1>bgm003\n<2>map023\n<3>sp0002h2\n<4>ev0001_l\n<5>black\n<6>Uma tarde de sexta-feira.\n<7>1-1_0712_Dream\n<8>São as últimas horas da semana.\n";
    let spec = parse_source(parser).unwrap();
    assert!(spec.line_indexed.is_some());
    assert!(spec
        .skip_rules
        .iter()
        .any(|rule| matches!(rule, kotoba_core::KotobaSkipRule::Matching(_))));
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 3);
    assert_eq!(entries[0].text, "Prólogo");
    assert_eq!(entries[1].text, "Uma tarde de sexta-feira.");
    assert_eq!(entries[2].text, "São as últimas horas da semana.");
}

#[test]
fn rust_runtime_accepts_recipe_numbered_voice_speaker_dialogue() {
    let parser = include_str!("../../../examples/subahibi_numbered_speaker_recipe.kotoba");
    let input = "<0>Prólogo\n<1>bgm003\n<2>yuki_000001\n<3>\"Haa...\"\n<4>yuki_000003\n<5>Yuki\n<6>\"Mamiya Takuji.\"\n<7>taku_000001\n<8>Takuji\n<9>\"Por que está aqui?\"\n<10>ev0001_l\n<11>Uma tarde de sexta-feira.\n";
    let spec = parse_source(parser).unwrap();
    assert!(spec.rules.iter().any(|r| r.name == "remember_speaker"));
    assert!(spec.rules.iter().any(|r| r.name == "dialogue_speaker"));
    assert!(spec
        .rules
        .iter()
        .any(|r| r.name == "dialogue_voice_speaker"));

    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 5);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].text, "Prólogo");
    assert_eq!(entries[1].kind, "dialogue");
    assert_eq!(entries[1].speaker.as_deref(), Some("yuki"));
    assert_eq!(entries[1].text, "Haa...");
    assert_eq!(entries[2].speaker.as_deref(), Some("Yuki"));
    assert_eq!(entries[2].text, "Mamiya Takuji.");
    assert_eq!(entries[3].speaker.as_deref(), Some("Takuji"));
    assert_eq!(entries[3].text, "Por que está aqui?");
    assert_eq!(entries[4].kind, "narration");
    assert_eq!(entries[4].text, "Uma tarde de sexta-feira.");

    let rebuilt = rebuild(
        input,
        &spec,
        &[KotobaPatchInput {
            id: entries[2].id.clone(),
            index: entries[2].index,
            source: entries[2].text.clone(),
            translation: "Mamiya Takuji!".into(),
            speaker_translation: String::new(),
        }],
    )
    .unwrap();
    assert!(rebuilt.contains("<6>\"Mamiya Takuji!\""));
}

#[test]
fn rust_runtime_speaker_lookahead_does_not_swallow_unquoted_fragments() {
    let parser = include_str!("../../../examples/subahibi_numbered_speaker_recipe.kotoba");
    let input = "<0>yuki_000002_b\n<1>cigarros,\n<2>ev0001_l\n<3>yuki_000002_c\n<4>um teto\n";
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 2);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].text, "cigarros,");
    assert_eq!(entries[1].kind, "narration");
    assert_eq!(entries[1].text, "um teto");
}

#[test]
fn rust_runtime_accepts_flexible_rule_angle_speaker() {
    let parser = include_str!("../../../examples/flexible_angle_speaker.kotoba");
    let input =
        "I wake up.\n<Natsuki>\"Somebody, save me!\"\nThe ship %i1Serpens Albus%id sinks.\n";
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 3);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[1].speaker.as_deref(), Some("Natsuki"));
    assert_eq!(entries[1].text, "Somebody, save me!");
    assert_eq!(entries[2].text, "The ship %i1Serpens Albus%id sinks.");

    let rebuilt = rebuild(
        input,
        &spec,
        &[KotobaPatchInput {
            id: entries[1].id.clone(),
            index: entries[1].index,
            source: entries[1].text.clone(),
            translation: "Alguém, me salve!".into(),
            speaker_translation: String::new(),
        }],
    )
    .unwrap();
    assert!(rebuilt.contains("<Natsuki>\"Alguém, me salve!\""));
}

#[test]
fn rust_runtime_accepts_flexible_rule_remembered_attribute_speaker() {
    let parser = include_str!("../../../examples/flexible_kirikiri_name.kotoba");
    let input = "[name text = \"Masayuki\"]\n\"Can I leave this box here?\"\n[tp]\n";
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 1);
    assert_eq!(entries[0].speaker.as_deref(), Some("Masayuki"));
    assert_eq!(entries[0].text, "Can I leave this box here?");
}

#[test]
fn rust_runtime_accepts_flexible_rule_tab_fields() {
    let parser = include_str!("../../../examples/flexible_printtext_tsv.kotoba");
    let input = "[CreateBG]=bg01;\n[PrintText]=00:01\tSekai\tWhat are you doing...?\t00:04;\n";
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 1);
    assert_eq!(entries[0].speaker.as_deref(), Some("Sekai"));
    assert_eq!(entries[0].text, "What are you doing...?");

    let rebuilt = rebuild(
        input,
        &spec,
        &[KotobaPatchInput {
            id: entries[0].id.clone(),
            index: entries[0].index,
            source: entries[0].text.clone(),
            translation: "O que você está fazendo...?".into(),
            speaker_translation: String::new(),
        }],
    )
    .unwrap();
    assert!(rebuilt.contains("[PrintText]=00:01\tSekai\tO que você está fazendo...?\t00:04;"));
}

#[test]
fn rust_runtime_flexible_binary_nut_filters_technical_symbols() {
    fn push_record(bytes: &mut Vec<u8>, text: &str) {
        bytes.extend_from_slice(&[0x10, 0x00, 0x00, 0x08]);
        bytes.extend_from_slice(&(text.as_bytes().len() as u32).to_le_bytes());
        bytes.extend_from_slice(text.as_bytes());
    }

    let parser = include_str!("../../../examples/flexible_binary_nut.kotoba");
    let mut input = Vec::new();
    push_record(&mut input, "media/script/lang/en/ma00_000.nut");
    push_record(&mut input, "main");
    push_record(&mut input, "TransText");
    push_record(&mut input, "Today's performance will consist of four acts.");
    push_record(&mut input, "We begin with a duel.");

    let spec = parse_source(parser).unwrap();
    let (entries, report) = kotoba_core::extract_bytes(&input, &spec).unwrap();

    assert_eq!(report.total_entries, 2);
    assert!(entries
        .iter()
        .all(|entry| !entry.text.contains("TransText")));
    assert!(entries
        .iter()
        .all(|entry| !entry.text.contains("media/script")));
    assert_eq!(
        entries[0].text,
        "Today's performance will consist of four acts."
    );
    assert_eq!(entries[1].text, "We begin with a duel.");
}

#[test]
fn rust_runtime_flexible_binary_nut_skips_markers_and_reads_speaker() {
    fn push_record(bytes: &mut Vec<u8>, text: &str) {
        bytes.extend_from_slice(&[0x10, 0x00, 0x00, 0x08]);
        bytes.extend_from_slice(&(text.as_bytes().len() as u32).to_le_bytes());
        bytes.extend_from_slice(text.as_bytes());
    }

    let parser = include_str!("../../../examples/flexible_binary_nut.kotoba");
    let mut input = Vec::new();
    push_record(&mut input, "media/script/lang/en/ma00_000.nut");
    push_record(&mut input, "main");
    push_record(&mut input, "TransText");
    push_record(
        &mut input,
        "\r\n  The two men square off at the center of the field.\r\n",
    );
    push_record(&mut input, "\r\n//【ｅｔｃ／落人】\r\n<voice name='ｅｔｃ／落人' class='その他男声' src='voice/ma00/0000080e286'>\r\n\"Very well.\r\n  So... what now?\"\r\n");
    push_record(
        &mut input,
        "\r\n  And there the warriors remain, motionless.\r\n",
    );
    push_record(&mut input, "<?>");

    let spec = parse_source(parser).unwrap();
    let (entries, report) = kotoba_core::extract_bytes(&input, &spec).unwrap();

    assert_eq!(report.total_entries, 4);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].speaker, None);
    assert_eq!(
        entries[0].text,
        "The two men square off at the center of the field."
    );
    assert_eq!(entries[1].kind, "dialogue");
    assert_eq!(entries[1].speaker.as_deref(), Some("ｅｔｃ／落人"));
    assert!(entries[1].text.contains("Very well"));
    assert_eq!(entries[2].kind, "dialogue");
    assert_eq!(entries[2].speaker.as_deref(), Some("ｅｔｃ／落人"));
    assert!(entries[2].text.contains("So... what now?"));
    assert_eq!(entries[3].kind, "narration");
    assert_eq!(entries[3].speaker, None);
    assert!(entries[3].text.contains("warriors remain"));
    assert!(entries
        .iter()
        .all(|entry| !entry.text.contains("TransText")));
    assert!(entries.iter().all(|entry| !entry.text.contains("<voice")));
    assert!(entries.iter().all(|entry| !entry.text.contains("//【")));
    assert!(entries.iter().all(|entry| entry.text.trim() != "<?>"));
}

#[test]
fn rust_runtime_accepts_composable_ef_recipe_without_message_or_rule_blocks() {
    let parser = r##"
parser EfComposableSC:
    file ".sc"
    encoding cp932

    protect:
        matches "\\\\[A-Za-z]+"
        matches "#[0-9]+"

    ignore:
        empty

    dialogue:
        when starts with ".message"
        capture id as number
        capture voice if like "AAA-*-0000"
        capture speaker before quoted text
        capture speaker before rest text if starts with any "@", "#"
        capture text as quoted or rest
        save as Dialogue
        save otherwise as Narration
        patch text
        patch speaker
"##;
    let input = concat!(
        ".message 100   I, Hiro Hirono, made my way through the frigid night.\n",
        ".message 140  Hiro “C-c-c-cold...”\n",
        ".message 230 yuk-100_01-0003 @Yuuko “Oh my.”\n",
        ".message 1860 miy-100_01-0005 #Miyako ＊sigh＊\n",
        ".message 1710 miy-100_01-1003  Was it going to snow?\n",
    );
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();

    assert_eq!(report.total_entries, 5);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].speaker, None);
    assert_eq!(
        entries[0].text,
        "I, Hiro Hirono, made my way through the frigid night."
    );
    assert_eq!(entries[1].kind, "dialogue");
    assert_eq!(entries[1].speaker.as_deref(), Some("Hiro"));
    assert_eq!(entries[1].text, "C-c-c-cold...");
    assert_eq!(entries[2].speaker.as_deref(), Some("Yuuko"));
    assert_eq!(entries[2].text, "Oh my.");
    assert_eq!(entries[3].speaker.as_deref(), Some("Miyako"));
    assert_eq!(entries[3].text, "＊sigh＊");
    assert_eq!(entries[4].kind, "narration");
    assert_eq!(entries[4].speaker, None);
    assert_eq!(entries[4].text, "Was it going to snow?");
}

#[test]
fn rust_runtime_gore_screaming_show_dialogue_block_extracts_once_and_roundtrips() {
    let parser = r#"
parser GoreScreamingShowSystemNNN:
    file ".txt"
    encoding utf8

    ignore:
        empty
        equals "/cut"

    choice:
        when starts with "/choice"
        capture text as rest
        save as Choice
        patch text

    dialogue:
        when starts with "<"
        capture speaker between "<" and ">"
        capture text after ">"
        save as Dialogue
        patch speaker
        patch text

    text:
        capture text as rest
        save as Narration
        patch text
"#;
    let input = concat!(
        "/cut\n",
        "/choice Go to the shrine\n",
        "<Kyoji>(That's Mt. Nishimori.)\n",
        "The sky hangs high above.\n",
        "\"...Help me.\"\n",
    );
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();

    assert_eq!(report.total_entries, 4);
    assert_eq!(entries[0].kind, "choice");
    assert_eq!(entries[0].text, "Go to the shrine");
    assert_eq!(entries[1].kind, "dialogue");
    assert_eq!(entries[1].speaker.as_deref(), Some("Kyoji"));
    assert_eq!(entries[1].text, "(That's Mt. Nishimori.)");
    assert_eq!(entries[2].kind, "narration");
    assert_eq!(entries[2].text, "The sky hangs high above.");
    assert_eq!(entries[3].kind, "narration");
    assert_eq!(entries[3].speaker, None);
    assert_eq!(entries[3].text, "...Help me.");

    let rebuilt = rebuild(
        input,
        &spec,
        &[
            KotobaPatchInput {
                id: entries[0].id.clone(),
                index: 0,
                source: entries[0].text.clone(),
                translation: "Ir ao santuário".into(),
                speaker_translation: String::new(),
            },
            KotobaPatchInput {
                id: entries[1].id.clone(),
                index: 1,
                source: entries[1].text.clone(),
                translation: "(Aquele é o Monte Nishimori.)".into(),
                speaker_translation: "Kyouji".into(),
            },
        ],
    )
    .unwrap();

    assert_eq!(
        rebuilt,
        concat!(
            "/cut\n",
            "/choice Ir ao santuário\n",
            "<Kyouji>(Aquele é o Monte Nishimori.)\n",
            "The sky hangs high above.\n",
            "\"...Help me.\"\n",
        )
    );
}

#[test]
fn named_semantic_blocks_replace_generic_rules_for_grisaia_shapes() {
    let parser = include_str!("../../../examples/grisaia_sce_cp932.kotoba");
    assert!(!parser
        .lines()
        .any(|line| line.trim_start().starts_with("rule ")));

    let input = concat!(
        "str 155 The promised place\n",
        "str 200 internal value\n",
        "Yuuji\t「A normal line.」\\@\n",
        "Amane\t「This line starts here\\@\n",
        "fg 100 amane\n",
        "and ends here.」\\@\n",
        "A narration line.\n",
    );
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();

    assert_eq!(report.total_entries, 5);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].text, "The promised place");
    assert_eq!(entries[1].kind, "dialogue");
    assert_eq!(entries[1].speaker.as_deref(), Some("Yuuji"));
    assert_eq!(entries[1].text, "A normal line.");
    assert_eq!(entries[2].speaker.as_deref(), Some("Amane"));
    assert_eq!(entries[2].text, "This line starts here");
    assert_eq!(entries[3].speaker.as_deref(), Some("Amane"));
    assert_eq!(entries[3].text, "and ends here.");
    assert_eq!(entries[4].kind, "narration");
    assert_eq!(entries[4].text, "A narration line.");
    assert!(entries.iter().all(|entry| entry.text != "internal value"));

    let rebuilt = rebuild(
        input,
        &spec,
        &[KotobaPatchInput {
            id: entries[1].id.clone(),
            index: entries[1].index,
            source: entries[1].text.clone(),
            translation: "Uma fala normal.".into(),
            speaker_translation: "Yuuji".into(),
        }],
    )
    .unwrap();
    assert!(rebuilt.contains("Yuuji\t「Uma fala normal.」\\@"));
}

#[test]
fn named_dialogue_keeps_specialized_dialogue_commands() {
    let parser = r#"
parser NamedEf:
    file ".sc"
    encoding cp932

    dialogue Message:
        when starts with ".message"
        capture id as number
        capture speaker before quoted text
        capture text as quoted
        patch speaker
        patch text
"#;
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(".message 10 Haruka \"Hello.\"\n", &spec).unwrap();
    assert_eq!(report.total_entries, 1);
    assert_eq!(entries[0].speaker.as_deref(), Some("Haruka"));
    assert_eq!(entries[0].text, "Hello.");
}

#[test]
fn rust_runtime_recipe_reads_stella_ast_blocks_with_section_voice_and_text() {
    let parser = r#"
parser StellaOfTheEndAST:
    file ".ast"
    encoding utf8

    read:
        records as blocks
        block starts with "text = {"
        block ends when braces close

    voice:
        section "vo"
        capture speaker from attribute "ch"
        capture voice from attribute "file"
        remember voice
        skip

    dialogue:
        section "en"
        capture text between "[[" and "]]"
        speaker fallback speaker
        save as Dialogue
        patch text
"#;
    let input = r#"text = {
    vo = {
        {"vo", ch="c001", file="z000100010"},
    },
    ja = {
        {
            "「クソっ、またか」",
        },
    },
    en = {
        {
            [["Great, another one."]],
        },
    },
    cn = {
        {
            "「该死，又来了」",
        },
    },
},
"#;
    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();
    assert_eq!(report.total_entries, 1);
    assert_eq!(entries[0].kind, "dialogue");
    assert_eq!(entries[0].speaker.as_deref(), Some("c001"));
    assert_eq!(entries[0].text, "Great, another one.");

    let rebuilt = rebuild(
        input,
        &spec,
        &[KotobaPatchInput {
            id: entries[0].id.clone(),
            index: 0,
            source: entries[0].text.clone(),
            translation: "Ótimo, mais um.".into(),
            speaker_translation: String::new(),
        }],
    )
    .unwrap();
    assert!(rebuilt.contains("[[\"Ótimo, mais um.\"]]"));
}

#[test]
fn rust_runtime_recipe_stella_quoted_narration_does_not_duplicate_wrapped_dialogue() {
    let parser = r#"
parser TsuinoStellaSiglusEngineAST:
    file ".ast"
    encoding utf8

    read:
        records as blocks
        block starts with "text = {"
        block ends when braces close

    ignore:
        empty

    voice:
        section "vo"
        capture speaker from attribute "ch"
        capture voice from attribute "file"
        remember voice
        skip

    dialogue:
        section "en"
        capture text between "[[" and "]]"
        speaker fallback voice.speaker
        save as Dialogue
        patch text

    text:
        section "en"
        capture text as quoted
        save as Narration
        patch text
"#;
    let input = r#"text = {
    en = {
        {
            "My data sensors register a faint tremor.",
        },
    },
},
text = {
    vo = {
        {"vo", ch="c001", file="z000100010"},
    },
    en = {
        {
            [["Great, another one."]],
        },
    },
},
"#;

    let spec = parse_source(parser).unwrap();
    let (entries, report) = extract(input, &spec).unwrap();

    assert_eq!(report.total_entries, 2);
    assert_eq!(entries[0].kind, "narration");
    assert_eq!(entries[0].text, "My data sensors register a faint tremor.");
    assert_eq!(entries[1].kind, "dialogue");
    assert_eq!(entries[1].speaker.as_deref(), Some("c001"));
    assert_eq!(entries[1].text, "Great, another one.");

    let rebuilt = rebuild(
        input,
        &spec,
        &[
            KotobaPatchInput {
                id: entries[0].id.clone(),
                index: entries[0].index,
                source: entries[0].text.clone(),
                translation: "Meus sensores de dados registram um leve tremor.".into(),
                speaker_translation: String::new(),
            },
            KotobaPatchInput {
                id: entries[1].id.clone(),
                index: entries[1].index,
                source: entries[1].text.clone(),
                translation: "Ótimo, mais um.".into(),
                speaker_translation: String::new(),
            },
        ],
    )
    .unwrap();

    assert!(rebuilt.contains("\"Meus sensores de dados registram um leve tremor.\""));
    assert!(rebuilt.contains("[[\"Ótimo, mais um.\"]]"));
}
