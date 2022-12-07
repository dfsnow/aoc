library(data.table)

# Reading and cleaning the input
crates <- readLines("5/input.txt", n = 8)
crates <- strsplit(gsub("(.{4})", "\\1|", crates), split = "\\|")
crates <- lapply(crates, function(x) gsub(" |\\[|\\]", "", x))
lst <- as.list(as.data.frame(t(as.data.frame(crates))))
lst2 <- lst <- lapply(lst, function(x) x[nzchar(x)])
df <- fread("5/input.txt", skip = 10)[, c(2, 4, 6)]

# Q1
for (i in as.list(as.data.frame(t(df)))) {
  lst[[i[3]]] <- c(rev(head(lst[[i[2]]], i[1])), lst[[i[3]]])
  lst[[i[2]]] <- lst[[i[2]]][-(1:i[1])]
}

print(paste0(sapply(lst, function(x) x[1]), collapse = ""))

# Q2
for (i in as.list(as.data.frame(t(df)))) {
  lst2[[i[3]]] <- c(head(lst2[[i[2]]], i[1]), lst2[[i[3]]])
  lst2[[i[2]]] <- lst2[[i[2]]][-(1:i[1])]
}

print(paste0(sapply(lst2, function(x) x[1]), collapse = ""))
