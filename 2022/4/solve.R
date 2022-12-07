library(data.table)

spl_conv <- function(x) lapply(strsplit(x, "-"), as.numeric)

range_check <- function(x, range_op, agg_op) {
  x[, (colnames(x)) := lapply(.SD, spl_conv), .SDcols = colnames(x)]
  x[, total := sign(
    mapply(function(x, y) agg_op(range_op(x, y)), V1, V2) +
    mapply(function(x, y) agg_op(range_op(x, y)), V2, V1)
  )]
  print(sum(x$total))
}


# Input
df <- fread("4/input.txt", header = FALSE)

# Q1
range_check(df, `%between%`, all)

# Q2
range_check(df, `%inrange%`, any)
