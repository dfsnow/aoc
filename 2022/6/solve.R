input <- readLines("6/input.txt", n = 8)

str_scan <- function(x, window = 4) {
  wm1 <- window - 1
  is_marker <- c()
  for (i in seq_len(nchar(input))) {
    is_marker <- c(
      is_marker,
      grepl("^(?!.*(.).*\\1)[a-z]+$", substr(input, i, i + wm1), perl = T)
    )
  }
  print(which(is_marker)[1] + wm1)
}

# Q1
str_scan(input, 4)

# Q2
str_scan(input, 14)
