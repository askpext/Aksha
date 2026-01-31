export interface SearchResult {
  path: string;
  name: string;
  extension: string;
  size: number;
  modified: number;
  score: number;
}

export interface IndexStatus {
  total_files: number;
  is_indexing: boolean;
}
