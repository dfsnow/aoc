top_crates <- function(x, directions, FUN) {
  for (i in as.list(as.data.frame(t(directions)))) {
    x[[i[3]]] <- c(FUN(head(x[[i[2]]], i[1])), x[[i[3]]])
    x[[i[2]]] <- x[[i[2]]][-(1:i[1])]
  }
  print(paste0(sapply(x, function(y) y[1]), collapse = ""))
}


# Input
crates <- readLines("5/input.txt", n = 8)
crates <- strsplit(gsub("(.{4})", "\\1|", crates), split = "\\|")
crates <- lapply(crates, function(x) gsub(" |\\[|\\]", "", x))
lst <- as.list(as.data.frame(t(as.data.frame(crates))))
lst <- lapply(lst, function(x) x[nzchar(x)])

df <- fread("5/input.txt", skip = 10)[, c(2, 4, 6)]

# Q1
top_crates(lst, df, rev)

# Q2
top_crates(lst, df, function(x) x)
