library(data.table)

# Input
df <- fread("1/input.txt")

# Q1
df[, grp := cumsum(is.na(V1))]
df <- df[, .(total = sum(V1, na.rm = TRUE)), by = grp]
print(max(df$total))

# Q2
setorder(df, -total)
print(df[1:3, total])
