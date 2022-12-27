library(data.table)

mflip <- function(x) matrix(rev(x), nrow(x), ncol(x))

from_edge <- function(x) {
  lag_x <- shift(x, type = "lag")
  highest_seen <- Reduce(max, x, accumulate = TRUE)
  first_seen <- highest_seen > shift(highest_seen, type = "lag")
  out <- (x > lag_x) & (x > highest_seen | first_seen)
  replace(out, is.na(out), FALSE)
}


from_tree <- function(vec, start = 1) {
  tree_val <- vec[start]
  out <- 0
  for (x in vec[(start + 1):length(vec)]) {
    out <- out + 1
    if (x >= tree_val) break
  }
  out
}


input <- readLines("8/input.txt")
input <- as.matrix(t(as.data.frame(sapply(strsplit(input, ""), as.integer))))
nc <- ncol(input)
nr <- nrow(input)

# Q1
top <- apply(input, 2, from_edge)
right <- mflip(t(apply(mflip(input), 1, from_edge)))
bottom <- mflip(apply(mflip(input), 2, from_edge))
left <- t(apply(input, 1, from_edge))

edges <- matrix(FALSE, nr, nc) 
edges[, 1] <- TRUE
edges[, nc] <- TRUE
edges[1, ] <- TRUE
edges[nr, ] <- TRUE

sum((sign(top + right + bottom + left) * !edges) + edges)

# Q2
up <- rbind(
  matrix(0, ncol = nc),
  t(mflip(mapply(function(x) apply(mflip(input), 2, from_tree, x), 1:(nc - 1))))
)
right <- cbind(
  mapply(function(x) t(apply(input, 1, from_tree, x)), 1:(nr - 1)),
  matrix(0, nrow = nr)
)
down <- rbind(
  t(mapply(function(x) apply(input, 2, from_tree, x), 1:(nc - 1))),
  matrix(0, ncol = nc)
)
left <- cbind(
  matrix(0, nrow = nr),
  mflip(mapply(function(x) t(apply(mflip(input), 1, from_tree, x)), 1:(nr - 1)))
)

max(up * right * down * left)
