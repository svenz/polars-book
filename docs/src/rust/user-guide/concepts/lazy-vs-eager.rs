use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    // --8<-- [start:eager]
    let df = CsvReader::from_path("docs/src/data/iris.csv").unwrap().finish().unwrap();
    let mask = df.column("sepal_width")?.f64()?.gt(5.0);
    let df_small = df.filter(&mask)?;
    let df_agg = df_small.groupby(["species"])?.select(["sepal_width"]).mean()?;
    println!("{}", df_agg);
    // --8<-- [end:eager]

    // --8<-- [start:lazy]
    let q = LazyCsvReader::new("docs/src/data/iris.csv")
        .has_header(true)
        .finish()?
        .filter(col("sepal_length").gt(lit(5)))
        .groupby(vec![col("species")])
        .agg([col("sepal_width").mean()]);
    let df = q.collect()?;
    println!("{}", df);
    // --8<-- [end:lazy]

    // --8<-- [start:streaming]
    let q = LazyCsvReader::new("docs/src/data/iris.csv")
        .has_header(true)
        .finish()?
        .filter(col("sepal_length").gt(lit(5)))
        .groupby(vec![col("species")])
        .agg([col("sepal_width").mean()]);
    
    let df = q.with_streaming(true).collect()?;
    println!("{}", df);
    // --8<-- [end:streaming]

    Ok(())
}