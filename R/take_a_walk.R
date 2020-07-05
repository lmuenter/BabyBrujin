#' Take A Walk
#'
#' Walk through the provided graph
#'
#' @param graph a directed graph (nested list) with nodes = names(list) and innodes/outnodes as sublists
#' @importFrom magrittr "%>%"
#' @export

#===============================================

walk_the_graph = function(graph){

  # get length of final path
  lpath = lapply(graph, "[[", 2) %>%
    unlist %>%
      na.omit %>%
        length + 1
  sprintf("path length = %s", lpath) %>% print

  # find start nodes
  starts = find_start_node(graph)
  sprintf("starting nodes found = %s", length(starts)) %>% print

  # generate path
  path = get_path(graph, 1, lpath, starts)
  print("path constructed successfully")
  return(path)
}

#--------------------------------------------
get_inOut_degrees = function(edges){
  sapply(seq_along(edges),
         function(i)
           length(edges[[i]]$innodes) - length(edges[[i]]$outnodes)
  )
}

#---------------------------------------------
find_start_node = function(edges){
  candidateNodes = which(get_inOut_degrees(edges) < 0)
  ifelse(
    length(candidateNodes) > 0,
    return(candidateNodes),
    seq_along(edges) %>%
      sample(., 1)
  )
}

#----------------------------------------------
get_path = function(graph, i, max, currNode, path){

  # at start, create path
  if(missing(path)){
    path = vector("numeric", max)
  }

  # iterate through the graph nNodes times
  if(i != max +1 ){
    nextNode = graph[[currNode]]$outnodes
    path[i] = currNode
    get_path(graph, i+1, max, nextNode, path)
  } else {
    return(path)
  }

}
