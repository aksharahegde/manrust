use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub fn filter_commands(query: &str, commands: &[String]) -> Vec<String> {
    if query.is_empty() {
        return commands.to_vec();
    }

    let matcher = SkimMatcherV2::default();
    let mut scored_commands: Vec<(i64, String)> = commands
        .iter()
        .filter_map(|cmd| {
            matcher.fuzzy_match(cmd, query).map(|score| (score, cmd.clone()))
        })
        .collect();

    scored_commands.sort_by(|a, b| b.0.cmp(&a.0));
    scored_commands.into_iter().map(|(_, cmd)| cmd).collect()
}

