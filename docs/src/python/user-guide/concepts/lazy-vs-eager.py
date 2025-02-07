import polars as pl

# --8<-- [start:eager]

df = pl.read_csv("docs/src/data/iris.csv")
df_small = df.filter(pl.col("sepal_length") > 5)
df_agg = df_small.groupby("species").agg(pl.col("sepal_width").mean())
print(df_agg)
# --8<-- [end:eager]

# --8<-- [start:lazy]
q = (
    pl.scan_csv("docs/src/data/iris.csv")
    .filter(pl.col("sepal_length") > 5)
    .groupby("species")
    .agg(pl.col("sepal_width").mean())
)

df = q.collect()
# --8<-- [end:lazy]

# --8<-- [start:streaming]
q = (
    pl.scan_csv("docs/src/data/iris.csv")
    .filter(pl.col("sepal_length") > 5)
    .groupby("species")
    .agg(pl.col("sepal_width").mean())
)

df = q.collect(streaming=True)
# --8<-- [end:streaming]
