use chrono::{DateTime, Utc};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub struct TelegramMessage {
    pub channel_id: i64,
    pub message_id: i32,
    pub date: DateTime<Utc>,
    pub text: String,
}
#[derive(Debug)]
pub enum MessageType {
    Signal,
    Update,
    Analysis,
    Promotion,
    Community,
    Media,
    Other,
}

#[derive(Debug, PartialEq)]
pub enum LifecycleEvent {
    Activation,
    StopManagement,
    ProfitManagement,
    Running,
    Cancellation,
    Closure,
    Other,
}

pub fn classify_lifecycle_message(text: &str) -> LifecycleEvent {
    let text = text.to_lowercase();

    if text.contains("active") {
        return LifecycleEvent::Activation;
    }

    if text.contains("stops at")
        || text.contains("stop at")
        || text.contains("sl at")
        || text.contains("sl ")
        || text.starts_with("sl")
    {
        return LifecycleEvent::StopManagement;
    }

    if text.contains("secure")
        || text.contains("tp1")
        || text.contains("tp2")
        || text.contains("tp3")
        || text.contains("break even")
        || text.contains("be now")
    {
        return LifecycleEvent::ProfitManagement;
    }

    if text.contains("running")
        || text.contains("pips")
    {
        return LifecycleEvent::Running;
    }

    if text.contains("delete")
        || text.contains("cancel")
        || text.contains("invalid")
    {
        return LifecycleEvent::Cancellation;
    }

    if text.contains("closed")
        || text.contains("close")
        || text.contains("done")
    {
        return LifecycleEvent::Closure;
    }

    LifecycleEvent::Other
}

#[test]
fn classify_lifecycle_messages() {
    assert_eq!(
        classify_lifecycle_message("Active ✅"),
        LifecycleEvent::Activation
    );

    assert_eq!(
        classify_lifecycle_message("Stops at 4045"),
        LifecycleEvent::StopManagement
    );

    assert_eq!(
        classify_lifecycle_message("Secure 50% and BE now"),
        LifecycleEvent::ProfitManagement
    );

    assert_eq!(
        classify_lifecycle_message("Running 70+pips"),
        LifecycleEvent::Running
    );

    assert_eq!(
        classify_lifecycle_message("Let's delete this for now"),
        LifecycleEvent::Cancellation
    );

    assert_eq!(
        classify_lifecycle_message("Trade closed"),
        LifecycleEvent::Closure
    );

    assert_eq!(
        classify_lifecycle_message("GOOD MORNING TEAM"),
        LifecycleEvent::Other
    );
}

#[test]
fn inspect_lifecycle_classification() {
    let messages = [
        (34, "Let's delete this for now ! Was looking for price to drop this area"),
        (35, "the other limits are valid for now"),
        (36, "Active"),
        (37, "Stops at 4045"),
        (38, "Secure half tp2 is almost here"),
        (41, "RUNNING 120PIPS"),
        (43, "WHAT A TRADEEE WE PLANNED THIS 9HRSS AGOOOO"),
        (44, "We're still holding with stops at 4045 secured 70%"),
        (55, "I'm going low risk on this!"),
        (56, "Active"),
        (58, "Secure 50% and BE now"),
        (59, "Running 70+pips"),
    ];

    for (message_id, text) in messages {
        let event = classify_lifecycle_message(text);

        println!("message {message_id}: {event:?}");
    }
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Single(f64),
    Range { low: f64, high: f64 },
}

#[derive(Debug, PartialEq)]
pub struct TradingSignal {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub entry: Entry,
    pub stop_loss: f64,
    pub take_profit: Option<f64>,
    pub tp1: Option<f64>,
    pub tp2: Option<f64>,
    pub tp3: Option<f64>,
}

pub fn parse_pair(text: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("Pair:") {
            return value.split_whitespace().next().map(str::to_string);
        }
    }

    None
}

#[test]
fn parses_pair() {
    let text = "Gold 🏆
Pair: XAUUSD 📊
Side: Short / Sell Limit
Entry: 4061
TP: Open
SL: 4072";

    let result = parse_pair(text);

    assert_eq!(result, Some("XAUUSD".to_string()));
}

pub fn parse_side(text: &str) -> Option<(String, String)> {
    for line in text.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("Side:") {
            let parts: Vec<&str> = value.split('/').map(str::trim).collect();

            if parts.len() == 2 {
                return Some((parts[0].to_string(), parts[1].to_string()));
            }
        }
    }

    None
}

#[test]
fn parses_side() {
    let text = "Gold 🏆
Pair: XAUUSD 📊
Side: Short / Sell Limit
Entry: 4061
TP: Open
SL: 4072";

    let result = parse_side(text);

    assert_eq!(
        result,
        Some(("Short".to_string(), "Sell Limit".to_string()))
    );
}

pub fn parse_entry(text: &str) -> Option<Entry> {
    for line in text.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("Entry:") {
            let numbers: Vec<f64> = value
                .split_whitespace()
                .filter_map(|part| part.parse::<f64>().ok())
                .collect();

            match numbers.as_slice() {
                [price] => return Some(Entry::Single(*price)),
                [low, high] => {
                    return Some(Entry::Range {
                        low: *low,
                        high: *high,
                    });
                }
                _ => return None,
            }
        }
    }

    None
}

#[test]
fn parses_single_entry() {
    let text = "Pair: XAUUSD
Side: Short / Sell Limit
Entry: 4061";

    let result = parse_entry(text);

    assert_eq!(result, Some(Entry::Single(4061.0)));
}

#[test]
fn parses_entry_range() {
    let text = "Pair: XAUUSD
Side: Short / Sell Limit
Entry: 4044 4049";

    let result = parse_entry(text);

    assert_eq!(
        result,
        Some(Entry::Range {
            low: 4044.0,
            high: 4049.0
        })
    );
}


pub fn parse_stop_loss(text: &str) -> Option<f64> {
    for line in text.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("SL:") {
            return value.trim().parse::<f64>().ok();
        }
    }

    None
}

#[test]
fn parses_stop_loss() {
    let text = "Pair: XAUUSD
Side: Short / Sell Limit
Entry: 4061
TP: Open
SL: 4072";

    let result = parse_stop_loss(text);

    assert_eq!(result, Some(4072.0));
}

pub fn parse_take_profit(text: &str) -> Option<f64> {
    for line in text.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("TP:") {
            return value.trim().parse::<f64>().ok();
        }
    }

    None
}

#[test]
fn parses_open_take_profit() {
    let text = "Pair: XAUUSD
Side: Short / Sell Limit
Entry: 4061
TP: Open
SL: 4072";

    let result = parse_take_profit(text);

    assert_eq!(result, None);
}

pub fn parse_tp_level(text: &str, level: u8) -> Option<f64> {
    let prefix = format!("TP{}", level);

    for line in text.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix(&prefix) {
            return value
                .split_whitespace()
                .next()
                .and_then(|value| value.parse::<f64>().ok());
        }
    }

    None
}

#[test]
fn parses_tp1() {
    let text = "Pair: XAUUSD
Side: Short / Sell Limit
Entry: 4061
TP: Open
SL: 4072
TP1 4057 40pips ✅
TP2 4051 100pips ✅
TP3 4046 150pips✅";

    let result = parse_tp_level(text, 1);

    assert_eq!(result, Some(4057.0));
}

#[test]
fn parses_tp2() {
    let text = "TP1 4057 40pips ✅
TP2 4051 100pips ✅
TP3 4046 150pips✅";

    let result = parse_tp_level(text, 2);

    assert_eq!(result, Some(4051.0));
}

#[test]
fn parses_tp3() {
    let text = "TP1 4057 40pips ✅
TP2 4051 100pips ✅
TP3 4046 150pips✅";

    let result = parse_tp_level(text, 3);

    assert_eq!(result, Some(4046.0));
}

pub fn parse_signal(text: &str) -> Option<TradingSignal> {
    let symbol = parse_pair(text)?;
    let (side, order_type) = parse_side(text)?;
    let entry = parse_entry(text)?;
    let stop_loss = parse_stop_loss(text)?;

    let take_profit = parse_take_profit(text);
    let tp1 = parse_tp_level(text, 1);
    let tp2 = parse_tp_level(text, 2);
    let tp3 = parse_tp_level(text, 3);

    Some(TradingSignal {
        symbol,
        side,
        order_type,
        entry,
        stop_loss,
        take_profit,
        tp1,
        tp2,
        tp3,
    })
}

#[test]
fn parses_complete_signal() {
    let text = "Gold 🏆
Pair: XAUUSD 📊
Side: Short / Sell Limit
Entry: 4061
TP: Open
SL: 4072
Note: past profits do not predict future profits 
Risk 0.5-1-2% 

TP1 4057 40pips ✅
TP2 4051 100pips ✅
TP3 4046 150pips✅

Use Proper Risk Management";

    let result = parse_signal(text).expect("Expected a valid signal");

    assert_eq!(result.symbol, "XAUUSD");
    assert_eq!(result.side, "Short");
    assert_eq!(result.order_type, "Sell Limit");
    assert_eq!(result.entry, Entry::Single(4061.0));
    assert_eq!(result.stop_loss, 4072.0);
    assert_eq!(result.take_profit, None);
    assert_eq!(result.tp1, Some(4057.0));
    assert_eq!(result.tp2, Some(4051.0));
    assert_eq!(result.tp3, Some(4046.0));
}

#[test]
fn rejects_non_signal() {
    let text = "Ready ?";

    let result = parse_signal(text);

    assert_eq!(result, None);
}

pub fn lifecycle_match_score(message_text: &str, signal: &TradingSignal) -> u8 {
    let text = message_text.to_lowercase();
    let mut score = 0;

    // Management language is a useful clue.
    if text.contains("active")
        || text.contains("running")
        || text.contains("secure")
        || text.contains("break even")
        || text.contains("be now")
        || text.contains("stops")
        || text.contains("stop")
    {
        score += 20;
    }

    // Look for prices mentioned in the message.
    let message_prices: Vec<f64> = text
        .split_whitespace()
        .filter_map(|word| {
            word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.')
                .parse::<f64>()
                .ok()
        })
        .collect();

    for price in message_prices {
        if (price - signal.stop_loss).abs() < 0.01 {
            score += 50;
        }

        if (price - signal.tp1.unwrap_or(f64::NAN)).abs() < 0.01 {
            score += 50;
        }

        if (price - signal.tp2.unwrap_or(f64::NAN)).abs() < 0.01 {
            score += 50;
        }

        if (price - signal.tp3.unwrap_or(f64::NAN)).abs() < 0.01 {
            score += 50;
        }

        match signal.entry {
            Entry::Single(entry) => {
                if (price - entry).abs() < 0.01 {
                    score += 30;
                }
            }
            Entry::Range { low, high } => {
                if price >= low && price <= high {
                    score += 30;
                }
            }
        }
    }

    score
}

#[test]
fn lifecycle_message_with_matching_price_scores_higher() {
    let signal = TradingSignal {
        symbol: "XAUUSD".to_string(),
        side: "Short".to_string(),
        order_type: "Sell Limit".to_string(),
        entry: Entry::Range {
            low: 4044.0,
            high: 4049.0,
        },
        stop_loss: 4060.0,
        take_profit: None,
        tp1: Some(4040.0),
        tp2: Some(4034.0),
        tp3: Some(4029.0),
    };

    let score = lifecycle_match_score("Stops at 4045", &signal);

    assert!(score > 0);
}

#[test]
fn unrelated_message_scores_zero() {
    let signal = TradingSignal {
        symbol: "XAUUSD".to_string(),
        side: "Short".to_string(),
        order_type: "Sell Limit".to_string(),
        entry: Entry::Range {
            low: 4044.0,
            high: 4049.0,
        },
        stop_loss: 4060.0,
        take_profit: None,
        tp1: Some(4040.0),
        tp2: Some(4034.0),
        tp3: Some(4029.0),
    };

    let score = lifecycle_match_score("GOOD MORNING TEAM", &signal);

    assert_eq!(score, 0);
}


#[test]
fn inspect_lifecycle_scores() {
    let signal = TradingSignal {
        symbol: "XAUUSD".to_string(),
        side: "Short".to_string(),
        order_type: "Sell Limit".to_string(),
        entry: Entry::Range {
            low: 4044.0,
            high: 4049.0,
        },
        stop_loss: 4060.0,
        take_profit: None,
        tp1: Some(4040.0),
        tp2: Some(4034.0),
        tp3: Some(4029.0),
    };

    let messages = [
        (34, "Let's delete this for now ! Was looking for price to drop this area"),
        (35, "the other limits are valid for now"),
        (36, "Active"),
        (37, "Stops at 4045"),
        (38, "Secure half tp2 is almost here"),
        (41, "RUNNING 120PIPS"),
        (43, "WHAT A TRADEEE WE PLANNED THIS 9HRSS AGOOOO"),
    ];

    for (message_id, text) in messages {
        let score = lifecycle_match_score(text, &signal);

        println!("message {message_id}: score = {score}");
    }
}


#[test]
fn parse_signal_accepts_valid_signal() {
    let text = r#"
Gold 🏆
Pair: XAUUSD 📊
Side: Short / Sell Limit
Entry: 4285 4290
TP: Open
SL: 4302
Note: past profits do not predict future profits
Risk 0.5-1-2%

TP1 4280 50pips ✅
TP2 4275 100pips ✅
TP3 4270 150pips ✅

Use Proper Risk Management
"#;

    let signal = parse_signal(text);

    assert!(signal.is_some());

    let signal = signal.unwrap();

    assert_eq!(signal.symbol, "XAUUSD");
    assert_eq!(signal.side, "Short");
    assert_eq!(signal.order_type, "Sell Limit");

    assert_eq!(
        signal.entry,
        Entry::Range {
            low: 4285.0,
            high: 4290.0,
        }
    );

    assert_eq!(signal.stop_loss, 4302.0);
    assert_eq!(signal.take_profit, None);
    assert_eq!(signal.tp1, Some(4280.0));
    assert_eq!(signal.tp2, Some(4275.0));
    assert_eq!(signal.tp3, Some(4270.0));
}

#[test]
fn parse_signal_rejects_normal_message() {
    let text = "RUNNING 120PIPS 🤑🤑🤑";

    assert_eq!(parse_signal(text), None);
}

#[test]
fn parse_signal_rejects_incomplete_signal() {
    let text = r#"
Gold 🏆
Pair: XAUUSD 📊
Side: Short / Sell Limit
Entry: 4285 4290
TP: Open
TP1 4280
TP2 4275
TP3 4270
"#;

    assert_eq!(parse_signal(text), None);
}