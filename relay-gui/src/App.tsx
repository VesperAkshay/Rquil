import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TreeView } from "./components/TreeView";
import { RequestEditor } from "./components/RequestEditor";
import "./App.css";

import { SecretsManager } from './components/SecretsManager';

function App() {
  const [requests, setRequests] = useState<string[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [environments, setEnvironments] = useState<string[]>([]);
  const [selectedEnv, setSelectedEnv] = useState<string>("");

  async function fetchRequests() {
    try {
      // Testing with the relative path to our examples
      const result: string[] = await invoke("list_requests", { path: "../../relay-core/examples" });
      setRequests(result);
      
      const envs: string[] = await invoke("get_environments", { path: "../../relay-core/examples" });
      setEnvironments(envs);
      if (envs.length > 0 && !selectedEnv) {
        setSelectedEnv(envs[0]);
      }
    } catch (error) {
      console.error(error);
    }
  }

  return (
    <main style={{ display: 'flex', height: '100vh', width: '100vw', margin: 0, overflow: 'hidden' }}>
      {/* Sidebar */}
      <aside style={{ width: '300px', borderRight: '1px solid #333', display: 'flex', flexDirection: 'column', background: '#1a1a1a' }}>
        <div style={{ padding: '15px', borderBottom: '1px solid #333', textAlign: 'center' }}>
          <h3 style={{ margin: '0 0 10px 0', color: '#fff' }}>Relay Collections</h3>
          <button onClick={fetchRequests} style={{ width: '100%', padding: '8px' }}>Load Examples</button>
        </div>
        {environments.length > 0 && (
          <div style={{ padding: '10px 15px', borderBottom: '1px solid #333' }}>
            <label style={{ display: 'block', color: '#888', fontSize: '0.8rem', marginBottom: '5px', textAlign: 'left' }}>Environment:</label>
            <select 
              value={selectedEnv} 
              onChange={e => setSelectedEnv(e.target.value)}
              style={{ width: '100%', padding: '8px', background: '#111', color: '#fff', border: '1px solid #444', borderRadius: '4px', cursor: 'pointer' }}
            >
              <option value="">-- No Environment --</option>
              {environments.map(e => <option key={e} value={e}>{e}</option>)}
            </select>
          </div>
        )}
        <SecretsManager collectionPath="../../relay-core/examples" />
        <div style={{ flex: 1, overflowY: 'auto' }}>
          {requests.length > 0 ? (
            <TreeView paths={requests} onSelect={setSelectedFile} />
          ) : (
            <p style={{ padding: '15px', color: '#666', textAlign: 'center', fontSize: '0.9rem' }}>
              No collection loaded.
            </p>
          )}
        </div>
      </aside>
      
      {/* Main Content Area */}
      <section style={{ flex: 1, padding: '30px', display: 'flex', flexDirection: 'column', background: '#242424', color: '#e0e0e0', overflowY: 'auto' }}>
        {selectedFile ? (
          <RequestEditor filePath={selectedFile} />
        ) : (
          <div style={{ display: 'flex', height: '100%', alignItems: 'center', justifyContent: 'center', color: '#666' }}>
            <p>Select a file from the sidebar to start editing</p>
          </div>
        )}
      </section>
    </main>
  );
}

export default App;
