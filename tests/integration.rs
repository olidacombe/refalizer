use color_eyre::Result;
use refalizer::{to_branch_name, RefError};

#[test]
fn test_empty() {
    assert!(to_branch_name("").is_err());
}

#[test]
fn test_no_possible_valid_output() {
    let inputs = [
        "/", "/.lock", "/./", "..", "~^:", "?*[", "@", // 9
    ];
    for bad in inputs {
        assert!(
            matches!(to_branch_name(bad), Err(RefError::NoValidOutput),),
            "`{bad}` should not produce a valid output"
        );
    }
}

#[test]
fn nice_conversions() -> Result<()> {
    let expectations = [
        ("no/.slash/dot", "no/slash/dot"),                       // 1
        ("no/dot./slash", "no/dot/slash"),                       // 1
        ("no end.lock", "no_end"),                               // 1
        ("no..double...dots", "no.double.dots"),                 // 3
        ("no\x20 octal 040 or \x7f 177", "no_octal_040_or_177"), // 4
        ("no~tilde^caret:colon", "no_tilde_caret_colon"),        // 4
        (
            "no?question-mark*asterisk[open-bracket",
            "no_question-mark_asterisk_open-bracket",
        ), // 5
        ("/no start slash", "no_start_slash"),                   // 6
        ("no end slash", "no_end_slash/"),                       // 6
        ("/no//consecutive///slashes////", "no_consecutive_slashes"), // 6
        ("no trailing dot.", "no_trailing_dot"),                 // 7
        ("no@{at-tash", "no_at-tash"),                           // 8
        ("no\\backslash", "no_backslash"),                       // 10
        ("spaces    unterscore", "spaces_underscore"),
        (
            "it's fine to be \"special\" (<3)",
            "it_s_fine_to_be_special_(<3)",
        ),
    ];

    for (input, expected_output) in expectations {
        let output = to_branch_name(input)?;
        assert_eq!(
            expected_output, output,
            "Wanted `{input}` -> `{expected_output}`, got `{output}`"
        );
    }

    Ok(())
}
