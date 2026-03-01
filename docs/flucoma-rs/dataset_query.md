# DataSetQuery

Module path: `flucoma_rs::data::{DataSetQuery, DataSetQueryResult, ComparisonOp, QueryCondition}`

## Types

- `ComparisonOp`
  - `Eq`
  - `Ne`
  - `Lt`
  - `Le`
  - `Gt`
  - `Ge`
- `QueryCondition { column, op, value, and_group }`
- `DataSetQueryResult { data: Vec<f64>, rows: usize, cols: usize, source_indices: Vec<usize> }`
- `DataSetQuery`

## API

```rust
pub fn execute(
    data: &[f64],
    rows: usize,
    cols: usize,
    selected_columns: &[usize],
    conditions: &[QueryCondition],
    limit: Option<usize>,
) -> Result<DataSetQueryResult, &'static str>;
```

## Notes

- Input matrix layout is row-major.
- `selected_columns` defines projected output column order.
- `and_group = true` contributes to AND evaluation; `false` contributes to OR evaluation.
- `source_indices` maps output rows back to source dataset row indices.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/datasetquery/+page.svx`.

- DataSetQuery filters and projects dataset rows using column-wise conditions.
- Querying can be used as a lightweight, structure-aware redistribution step before downstream analysis.
- Limiting results is useful for bounded audition or UI workflows.
