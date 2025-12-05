pub fn solve5(content: &str) -> crate::Solutions {
    let mut _ids: Vec<u64> = Vec::new();
    let mut good_ids: Vec<(u64, u64)> = Vec::new();
    let mut not_spoiled: Vec<u64> = Vec::new();
    {
        let data: Vec<&str> = content.lines().collect();
        let empty_idx = data
            .iter()
            .position(|&line| line.is_empty())
            .unwrap_or(data.len());
        let rules = &data[..empty_idx];
        for rule in rules {
            let parts: Vec<&str> = rule.split("-").collect();
            let min_value = parts[0].parse::<u64>().unwrap();
            let max_value = parts[1].parse::<u64>().unwrap();
            good_ids.push((min_value, max_value));
        }
        good_ids = merge_ranges(&mut good_ids);

        _ids = data[empty_idx + 1..]
            .iter()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
    }
    for id in _ids {
        if in_ranges(id, &good_ids) {
            not_spoiled.push(id);
        }
    }
    let mut total_good_ids_count = 0;
    for (min_value, max_value) in &good_ids {
        total_good_ids_count += max_value - min_value + 1;
    }
    crate::Solutions::Five {
        result1: not_spoiled.len() as u32,
        good_ids: not_spoiled,
        total_good_ids_count: total_good_ids_count,
        final_ranges: good_ids,
    }
}

fn in_ranges(id: u64, ranges: &[(u64, u64)]) -> bool {
    for (min_value, max_value) in ranges {
        if id >= *min_value && id <= *max_value {
            return true;
        }
    }
    false
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }
    // Sort ranges by starting value
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];
    for &range in &ranges[1..] {
        if range.0 <= current.1 + 1 {
            // Overlapping or contiguous ranges, merge them
            current.1 = current.1.max(range.1);
        } else {
            // No overlap, push the current range and start a new one
            merged.push(current);
            current = range;
        }
    }
    // Push the last range
    merged.push(current);
    merged
}

#[cfg(test)]
mod test5 {
    use super::*;

    #[test]
    fn test_merge_ranges() {
        assert_eq!(
            merge_ranges(vec![(1, 3), (2, 4), (6, 8), (7, 10)].as_mut()),
            vec![(1, 4), (6, 10)]
        );
        assert_eq!(merge_ranges(vec![].as_mut()), vec![]);
        assert_eq!(
            merge_ranges(vec![(5, 7), (1, 3), (4, 6)].as_mut()),
            vec![(1, 7)]
        );
        assert_eq!(
            merge_ranges(
                vec![
                    (500789379744360, 501178637407391),
                    (81573181850686, 86374068906214),
                    (274689463122259, 278436146129529),
                    (263163202419245, 263690419295362),
                    (64627107934678, 66292153896231),
                    (493906974314704, 494688955465215),
                    (494688955465216, 500789379744359)
                ]
                .as_mut()
            ),
            vec![
                (64627107934678, 66292153896231),
                (81573181850686, 86374068906214),
                (263163202419245, 263690419295362),
                (274689463122259, 278436146129529),
                (493906974314704, 501178637407391),
            ]
        );
    }
}
