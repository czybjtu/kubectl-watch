fn print_diff_result() {
    if summary {
        if print_unchanged {
        }
    }

    let opposite_to_lhs = opposite_positions(&summary.lhs_positions);

    let hunks = merge_adjacent(
        hunks,
        opposite_to_lhs,
    );

    let lang_name;
}
