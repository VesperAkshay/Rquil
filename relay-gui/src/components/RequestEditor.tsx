import { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";

export interface SendResponse {
  status: number;
  headers: Record<string, string>;
  body: string;
  time_ms: number;
}

export interface RelayFile {
  meta: any;
  request: {
    method: string;
    url: string;
    headers: Record<string, string>;
    body: { type: string, content: string } | null;
  };
  script: any;
}

export function RequestEditor({ filePath }: { filePath: string }) {
  const [method, setMethod] = useState("GET");
  const [url, setUrl] = useState("https://httpbin.org/get");
  const [headers, setHeaders] = useState("");
  const [body, setBody] = useState("");
  
  const [response, setResponse] = useState<SendResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadFile() {
      try {
        const fileData = await invoke<RelayFile>("load_request", { path: filePath });
        setMethod(fileData.request.method);
        setUrl(fileData.request.url);
        
        // Pretty print headers if present
        const reqHeaders = fileData.request.headers || {};
        if (Object.keys(reqHeaders).length > 0) {
          setHeaders(JSON.stringify(reqHeaders, null, 2));
        } else {
          setHeaders("");
        }

        if (fileData.request.body) {
          setBody(fileData.request.body.content);
        } else {
          setBody("");
        }
        
        setResponse(null);
        setError(null);
      } catch (e: any) {
        console.error("Failed to load file:", e);
        setError(`Failed to load request file: ${e}`);
      }
    }
    loadFile();
  }, [filePath]);

  async function handleSend() {
    setLoading(true);
    setError(null);
    setResponse(null);
    
    let parsedHeaders = {};
    if (headers.trim() !== "") {
      try {
        parsedHeaders = JSON.parse(headers);
      } catch (e: any) {
        setError(`Failed to parse headers as JSON: ${e.message}`);
        setLoading(false);
        return;
      }
    }

    try {
      const res = await invoke<SendResponse>("send_request", {
        req: {
          method,
          url,
          headers: parsedHeaders,
          body: body.trim() !== "" ? body : null,
        }
      });
      setResponse(res);
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e.message || "Unknown error");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '20px', width: '100%', maxWidth: '900px', margin: '0 auto', textAlign: 'left' }}>
      <div>
        <h2 style={{ borderBottom: '1px solid #444', paddingBottom: '10px', margin: '0 0 10px 0' }}>Request Editor</h2>
        <div style={{ color: '#888', fontSize: '0.9rem' }}>Editing: <code style={{ color: '#aaa' }}>{filePath}</code></div>
      </div>
      
      <div style={{ display: 'flex', gap: '10px' }}>
        <select 
          value={method} 
          onChange={e => setMethod(e.target.value)}
          style={{ padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontSize: '1rem', cursor: 'pointer' }}
        >
          <option>GET</option>
          <option>POST</option>
          <option>PUT</option>
          <option>PATCH</option>
          <option>DELETE</option>
        </select>
        
        <input 
          type="text" 
          value={url} 
          onChange={e => setUrl(e.target.value)} 
          placeholder="https://api.example.com/v1/users"
          style={{ flex: 1, padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontSize: '1rem' }}
        />
        
        <button 
          onClick={handleSend}
          disabled={loading}
          style={{ padding: '12px 24px', background: loading ? '#555' : '#3b82f6', color: '#fff', border: 'none', borderRadius: '6px', cursor: loading ? 'wait' : 'pointer', fontWeight: 'bold', fontSize: '1rem', minWidth: '100px' }}
        >
          {loading ? "..." : "Send"}
        </button>
      </div>

      <div style={{ display: 'flex', gap: '20px' }}>
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <label style={{ fontWeight: 'bold', color: '#ccc' }}>Headers (JSON format for now):</label>
          <textarea 
            value={headers} 
            onChange={e => setHeaders(e.target.value)} 
            placeholder='{ "Content-Type": "application/json" }'
            style={{ height: '150px', padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontFamily: 'monospace', fontSize: '0.9rem', resize: 'vertical' }}
          />
        </div>

        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <label style={{ fontWeight: 'bold', color: '#ccc' }}>Body:</label>
          <textarea 
            value={body} 
            onChange={e => setBody(e.target.value)} 
            placeholder='{ "key": "value" }'
            style={{ height: '150px', padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontFamily: 'monospace', fontSize: '0.9rem', resize: 'vertical' }}
          />
        </div>
      </div>

      {error && (
        <div style={{ padding: '15px', background: '#ef444433', border: '1px solid #ef4444', borderRadius: '6px', color: '#fca5a5' }}>
          <strong>Error: </strong> {error}
        </div>
      )}

      {response && (
        <div style={{ marginTop: '20px', borderTop: '2px solid #444', paddingTop: '20px' }}>
          <h2 style={{ margin: '0 0 15px 0' }}>Response</h2>
          <div style={{ display: 'flex', gap: '15px', marginBottom: '15px' }}>
            <span style={{ padding: '6px 12px', background: response.status >= 200 && response.status < 300 ? '#22c55e33' : '#ef444433', color: response.status >= 200 && response.status < 300 ? '#4ade80' : '#fca5a5', borderRadius: '4px', fontWeight: 'bold' }}>
              Status: {response.status}
            </span>
            <span style={{ padding: '6px 12px', background: '#333', borderRadius: '4px', color: '#ccc' }}>
              Time: {response.time_ms} ms
            </span>
          </div>
          
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginBottom: '15px' }}>
            <label style={{ fontWeight: 'bold', color: '#ccc' }}>Response Headers:</label>
            <pre style={{ background: '#1a1a1a', padding: '12px', borderRadius: '6px', border: '1px solid #444', overflowX: 'auto', margin: 0, fontSize: '0.85rem' }}>
              {Object.entries(response.headers).map(([k, v]) => `${k}: ${v}`).join('\n')}
            </pre>
          </div>

          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            <label style={{ fontWeight: 'bold', color: '#ccc' }}>Response Body:</label>
            <textarea 
              readOnly
              value={response.body}
              style={{ height: '300px', padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontFamily: 'monospace', fontSize: '0.9rem', resize: 'vertical' }}
            />
          </div>
        </div>
      )}
    </div>
  );
}
