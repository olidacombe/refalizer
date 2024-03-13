use color_eyre::Result;
use refalizer::{to_branch_name, RefError};

#[test]
fn test_empty() {
    assert!(to_branch_name("").is_err());
}

#[test]
fn test_no_possible_valid_output() {
    let inputs = ["/", "/.lock", "/./", "..", "~^:", "?*["];
    for bad in inputs {
        assert!(
            matches!(to_branch_name(bad), Err(RefError::NoValidOutput),),
            "`{bad}` should not produce a valid output"
        );
    }
}

#[test]
fn nice_conversions() -> Result<()> {
    let expectations = [("foo bar", "foo_bar")];

    for (input, expected_output) in expectations {
        let output = to_branch_name(input)?;
        assert_eq!(
            expected_output, output,
            "Wanted `{input}` -> `{expected_output}`, got `{output}`"
        );
    }

    Ok(())
}
