#' @include get_nodes.R
NULL

#' Get nodes
#'
#' Generates nodes
#'
#' @param reads A set of "reads" to be processed (A vector of strings)
#' @param k Subsequence length, integer. must be smaller than sequence length
#' @importFrom magrittr %>%
#' @export

get_nodes = function(reads, k){

  ## determine nodes and edges
  kmer.ls         = lapply(reads, FUN=get_kmers, k = k)
  find_unique_kmers(kmer.ls)

}

find_unique_kmers = function(kmer.ls){
  unique_kmers = Reduce(c, kmer.ls) %>%
    unique
}
