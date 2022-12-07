input <- readLines("6/input.txt")

str_scan <- function(x, ws = 4) {
  for (i in seq_len(nchar(x))) {
      if (grepl("^(?!.*(.).*\\1)[a-z]+$", substr(x, i, i + ws - 1), perl = T)) {
        print(i + ws - 1)
        break
      }
  }
}

# Q1
str_scan(input, 4)

# Q2
str_scan(input, 14)
