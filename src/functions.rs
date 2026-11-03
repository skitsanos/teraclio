use chrono::{Local, SecondsFormat, Utc};
use tera::{Kwargs, State, TeraResult, Value};

/// Custom template functions: name and `--list-filters` description.
pub const FUNCTIONS: &[(&str, &str)] = &[(
    "now",
    "Current time as RFC 3339 (args: utc=false, timestamp=false)",
)];

/**
 * `now()`: the current time. Tera 2 dropped Tera 1's built-in, which most
 * "generated at" stamps relied on. Returns an RFC 3339 string in local time
 * (or UTC with `utc=true`) that `date_format` parses directly, or Unix
 * seconds with `timestamp=true`.
 * @author: skitsanos
 */
pub fn now(kwargs: Kwargs, _: &State) -> TeraResult<Value> {
    if kwargs.get::<bool>("timestamp")?.unwrap_or(false) {
        return Ok(Value::from(Utc::now().timestamp()));
    }
    let stamp = if kwargs.get::<bool>("utc")?.unwrap_or(false) {
        Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
    } else {
        Local::now().to_rfc3339_opts(SecondsFormat::Secs, false)
    };
    Ok(Value::from(stamp))
}

#[cfg(test)]
mod tests {
    use tera::{Context, Tera};

    fn render(template: &str) -> String {
        let mut tera = Tera::default();
        tera.register_function("now", super::now);
        tera.add_raw_template("t", template).unwrap();
        tera.render("t", &Context::new()).unwrap()
    }

    #[test]
    fn now_is_rfc3339() {
        for template in ["{{ now() }}", "{{ now(utc=true) }}"] {
            let stamp = render(template);
            assert!(
                chrono::DateTime::parse_from_rfc3339(&stamp).is_ok(),
                "{template} -> {stamp}"
            );
        }
        assert!(render("{{ now(utc=true) }}").ends_with('Z'));
    }

    #[test]
    fn now_timestamp_is_unix_seconds() {
        let seconds: i64 = render("{{ now(timestamp=true) }}").parse().unwrap();
        assert!((seconds - chrono::Utc::now().timestamp()).abs() < 5);
    }
}
