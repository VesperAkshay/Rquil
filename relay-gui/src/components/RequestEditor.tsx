import { useState } from 'react';

export function RequestEditor({ filePath }: { filePath: string }) {
  const [method, setMethod] = useState("GET");
  const [url, setUrl] = useState("https://");
  const [headers, setHeaders] = useState("");
  const [body, setBody] = useState("");

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
        
        <button style={{ padding: '12px 24px', background: '#3b82f6', color: '#fff', border: 'none', borderRadius: '6px', cursor: 'pointer', fontWeight: 'bold', fontSize: '1rem' }}>
          Send
        </button>
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <label style={{ fontWeight: 'bold', color: '#ccc' }}>Headers (JSON format for now):</label>
        <textarea 
          value={headers} 
          onChange={e => setHeaders(e.target.value)} 
          placeholder='{ "Content-Type": "application/json" }'
          style={{ height: '100px', padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontFamily: 'monospace', fontSize: '0.9rem', resize: 'vertical' }}
        />
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <label style={{ fontWeight: 'bold', color: '#ccc' }}>Body:</label>
        <textarea 
          value={body} 
          onChange={e => setBody(e.target.value)} 
          placeholder='{ "key": "value" }'
          style={{ height: '250px', padding: '12px', background: '#1a1a1a', color: '#fff', border: '1px solid #444', borderRadius: '6px', fontFamily: 'monospace', fontSize: '0.9rem', resize: 'vertical' }}
        />
      </div>
    </div>
  );
}
