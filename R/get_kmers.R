#' Get kmers
#'
#' A function to build k-mers from a sequence given a length k.
#'
#' @param sequence String to be subsetted
#' @param k Subsequence size, integer. must be smaller than sequence length
#' @export

get_kmers = function(sequence, k){
  "Builds subsequences of length k
   Inspiration: kmer-construction: https://stackoverflow.com/questions/21737971/generating-substrings-and-random-strings-in-r"
  starts = seq(nchar(sequence) - k + 1)
  mapply(FUN=substr, start = starts, stop = starts + k -1, x = list(sequence))
}
