input <- readLines("7/input.txt")
input <- input[!(grepl("^dir ", input) | grepl("\\$ ls", input))]

paths <- list()
for (line in input) {
  if (!startsWith(line, "$")) {
    size <- as.integer(gsub("[^0-9]", "", line))
    if (is.null(paths[[path_str]])) {
      paths <- c(paths, setNames(list(integer(0)), path_str))
    }
    paths[[path_str]] <- c(paths[[path_str]], size)
  }
  
  if (line == "$ cd /") {
    path <- "/"
  } else if (line == "$ cd ..") {
    path <- path[-length(path)]
  } else if (startsWith(line, "$ cd ")) {
    path <- c(path, paste0(gsub("\\$ cd ", "", line), "/"))
  }
  path_str <- paste0(path, collapse = "")
}

# Collapse path sizes up the tree
for (x in names(paths)) {
  sub_paths <- paths[names(paths) != x]
  contains_path <- startsWith(names(sub_paths), x)
  if (any(contains_path)) {
    paths[[x]] <- c(paths[[x]], unlist(sub_paths[contains_path]))
  }
}

# Q1
path_totals <- lapply(paths, sum, na.rm = TRUE)
sum(as.numeric(path_totals[path_totals <= 100000]))

# Q2
used <- unlist(path_totals)
target <- 3e7 - (7e7 - used["/"])
used[used >= target][which.min(used[used >= target])]
