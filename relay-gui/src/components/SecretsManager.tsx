import { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";

export function SecretsManager({ collectionPath }: { collectionPath: string }) {
  const [secrets, setSecrets] = useState<{key: string, value: string, visible: boolean}[]>([]);
  const [isExpanded, setIsExpanded] = useState(false);

  async function fetchSecrets() {
    try {
      const data: Record<string, string> = await invoke("get_secrets", { path: collectionPath });
      const mapped = Object.entries(data).map(([key, value]) => ({
        key,
        value,
        visible: false
      }));
      setSecrets(mapped.sort((a, b) => a.key.localeCompare(b.key)));
    } catch (e) {
      console.error("Failed to load secrets:", e);
    }
  }

  useEffect(() => {
    fetchSecrets();
  }, [collectionPath]);

  function toggleVisible(key: string) {
    setSecrets(secrets.map(s => s.key === key ? { ...s, visible: !s.visible } : s));
  }

  if (secrets.length === 0) return null;

  return (
    <div style={{ padding: '10px 15px', borderBottom: '1px solid #333' }}>
      <div 
        style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', cursor: 'pointer' }}
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <label style={{ color: '#888', fontSize: '0.8rem', margin: 0, cursor: 'pointer' }}>Secrets</label>
        <span style={{ color: '#555', fontSize: '0.8rem' }}>{isExpanded ? '▼' : '▶'}</span>
      </div>
      
      {isExpanded && (
        <div style={{ marginTop: '10px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
          {secrets.map(secret => (
            <div key={secret.key} style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', background: '#111', padding: '6px 8px', borderRadius: '4px', border: '1px solid #444' }}>
              <span style={{ color: '#ccc', fontSize: '0.8rem', fontWeight: 'bold' }}>{secret.key}</span>
              <div style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
                <span style={{ color: secret.visible ? '#fff' : '#666', fontSize: '0.8rem', fontFamily: 'monospace' }}>
                  {secret.visible ? secret.value : '••••••••'}
                </span>
                <button 
                  onClick={(e) => { e.stopPropagation(); toggleVisible(secret.key); }}
                  style={{ background: 'transparent', border: 'none', color: '#888', cursor: 'pointer', padding: '0 4px' }}
                  title={secret.visible ? "Hide" : "Show"}
                >
                  {secret.visible ? '👁️‍🗨️' : '👁️'}
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
