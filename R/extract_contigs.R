#' Extract contigs
#'
#' Extracts contigs from nodes according to the provided path.
#'
#' @param path a path (character vector) with order of nodes to be extracted.
#' @return A reconstructed sentence (character vector)
#' @importFrom magrittr "%>%"
#' @importFrom stringr str_c
#' @importFrom stringr str_sub
#' @export

get_contigs = function(path, nodes){
  startingNode=nodes[path[1]]
  followingNodes=nodes[path[-1]]

  addedChars = sapply(followingNodes, str_sub, start = -1L, end = -1L) %>%
    str_c(., collapse="")
  out = str_c(startingNode, addedChars, collapse = TRUE)
  return(out)
}
