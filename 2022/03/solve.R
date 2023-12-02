library(data.table)

conv_to_pri <- function(x) {
  match(x, c(letters[1:26], toupper(letters[1:26])))
}


# Q1
df <- fread("3/input.txt", header = FALSE)

df[, c("L", "R") := .(
  strsplit(substr(V1, 1, nchar(V1) / 2), split = ""),
  strsplit(substr(V1, (nchar(V1) / 2) + 1, length(V1)), split = "")
)]
df[, matched := mapply(function(l, r) l %in% r, L, R)]
df[, matched := conv_to_pri(mapply(function(x, y) unique(x[y]), L, matched))]

print(sum(df$matched))

# Q2
df <- fread("3/input.txt", header = FALSE)

df[, V1 := strsplit(V1, split = "")]
df[, c("id", "grp") := .(
  rep(1:100, each = 3),
  paste0("G", rep(1:3, times = 100))
)]
df <- dcast(df, id ~ grp, value.var = "V1")
df[, matched := mapply(function(x, y, z) x %in% y & x %in% z, G1, G2, G3)]
df[, matched := conv_to_pri(mapply(function(x, y) unique(x[y]), G1, matched))]

print(sum(df$matched))
