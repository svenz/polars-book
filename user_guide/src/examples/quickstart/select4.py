from .dataframe2 import df
import polars as pl

out = df.select([pl.exclude("a")])
