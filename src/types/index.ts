export interface Project {
  name: string;
  path: string;
  isProject: boolean;
  hasReadme: boolean;
  description?: string;
  version?: string;
  lastModified?: Date;
}

export interface Group {
  name: string;
  path: string;
  isProject: false;
  children: Project[];
}

export interface TreeItem extends Project {
  children?: TreeItem[];
  isExpanded?: boolean;
}

export interface GitInfo {
  branch: string;
  lastCommit: string;
  lastCommitAuthor: string;
  lastCommitTime: Date;
  status: 'clean' | 'modified' | 'untracked';
}

export interface ProjectConfig {
  scanPath: string;
  theme: 'light' | 'dark' | 'auto';
  sidebarWidth: number;
}
