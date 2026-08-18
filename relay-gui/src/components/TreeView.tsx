export interface TreeNode {
  name: string;
  fullPath: string;
  isDirectory: boolean;
  children: { [key: string]: TreeNode };
}

export function TreeView({ paths, onSelect }: { paths: string[], onSelect: (path: string) => void }) {
  const commonPrefix = getCommonPrefix(paths);
  
  const root: TreeNode = { name: 'root', fullPath: '', isDirectory: true, children: {} };

  paths.forEach(p => {
    // Normalize path separators
    const normalized = p.replace(/\\/g, '/');
    const relative = normalized.substring(commonPrefix.length);
    const parts = relative.split('/');
    
    let current = root;
    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      if (!current.children[part]) {
        current.children[part] = {
          name: part,
          fullPath: i === parts.length - 1 ? p : '', // Only leaf nodes get the clickable full path
          isDirectory: i < parts.length - 1,
          children: {}
        };
      }
      current = current.children[part];
    }
  });

  return (
    <div className="tree-view" style={{ textAlign: 'left', padding: '10px', color: '#ccc' }}>
      <TreeNodes nodes={Object.values(root.children)} onSelect={onSelect} />
    </div>
  );
}

function TreeNodes({ nodes, onSelect }: { nodes: TreeNode[], onSelect: (path: string) => void }) {
  return (
    <ul style={{ listStyleType: 'none', paddingLeft: '15px', margin: 0 }}>
      {nodes.map(node => (
        <li key={node.name} style={{ margin: '4px 0' }}>
          {node.isDirectory ? (
            <details open>
              <summary style={{ cursor: 'pointer', fontWeight: 'bold', userSelect: 'none' }}>
                📁 {node.name}
              </summary>
              <TreeNodes nodes={Object.values(node.children)} onSelect={onSelect} />
            </details>
          ) : (
            <div 
              style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', userSelect: 'none' }} 
              onClick={() => onSelect(node.fullPath)}
            >
              📄 {node.name}
            </div>
          )}
        </li>
      ))}
    </ul>
  );
}

function getCommonPrefix(paths: string[]): string {
    if (paths.length === 0) return "";
    let prefix = paths[0].replace(/\\/g, '/');
    for (let i = 1; i < paths.length; i++) {
        const p = paths[i].replace(/\\/g, '/');
        let j = 0;
        while (j < prefix.length && j < p.length && prefix[j] === p[j]) {
            j++;
        }
        prefix = prefix.substring(0, j);
    }
    // Snap back to the last complete folder
    const lastSlash = prefix.lastIndexOf('/');
    if (lastSlash !== -1) {
        prefix = prefix.substring(0, lastSlash + 1);
    }
    return prefix;
}
