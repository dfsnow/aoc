library(data.table)

df <- fread("2022/1/input.txt")
df[, grp := cumsum(is.na(V1))]
df <- df[, .(total = sum(V1, na.rm = TRUE)), by = grp]

# Q1
print(max(df$total))

# Q2
setorder(df, -total)
print(df[1:3, total])
