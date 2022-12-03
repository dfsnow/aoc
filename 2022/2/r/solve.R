library(data.table)

conv_to_scores <- function(x) {
  fcase(
    x %in% c("X", "A"), 1,
    x %in% c("Y", "B"), 2,
    x %in% c("Z", "C"), 3
  )
}


# Q1
df <- fread(
  file = "2022/2/input.txt",
  sep = " ",
  header = FALSE,
  col.names = c("them", "me")
)

df[, me := conv_to_scores(me)][, them := conv_to_scores(them)]
df[, outcome := fcase(
  (me - them) %in% c(1, -2), 6,
  me == them, 3,
  default = 0
)]
df[, final_score := outcome + me]

print(sum(df$final_score))

# Q2
df <- fread(
  file = "2022/2/input.txt",
  sep = " ",
  header = FALSE,
  col.names = c("them", "outcome")
)

df[, them := conv_to_scores(them)]
df[, outcome := fcase(
  outcome == "X", 0,
  outcome == "Y", 3,
  outcome == "Z", 6
)][, me := fcase(
  outcome == 0, ifelse(them - 1 < 1, 3, them - 1),
  outcome == 3, them,
  outcome == 6, ifelse(them + 1 > 3, 1, them + 1)
)]
df[, final_score := outcome + me]

print(sum(df$final_score))
