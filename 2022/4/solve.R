library(data.table)

spl_conv <- function(x) lapply(strsplit(x, "-"), as.numeric)


# Q1
df <- fread("4/input.txt", header = FALSE)

df[, (colnames(df)) := lapply(.SD, spl_conv), .SDcols = colnames(df)]
df[, total := sign(
  mapply(function(x, y) all(x %between% y), V1, V2) +
  mapply(function(x, y) all(x %between% y), V2, V1)
)]

print(sum(df$total))

# Q2
df[, total := sign(
  mapply(function(x, y) any(x %inrange% y), V1, V2) +
  mapply(function(x, y) any(x %inrange% y), V2, V1)
)]

print(sum(df$total))
