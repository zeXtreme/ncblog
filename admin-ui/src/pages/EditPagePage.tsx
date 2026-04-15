import { useState, useEffect, useRef, FormEvent } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { api, Page } from '../api'
import MilkdownEditor, { MilkdownEditorHandle } from '../components/MilkdownEditor'
import './EditPostPage.css'

const PAGE_LABELS: Record<string, string> = {
  about: '关于',
  archives: '归档',
}

export default function EditPagePage() {
  const { name } = useParams<{ name: string }>()
  const navigate = useNavigate()
  const editorRef = useRef<MilkdownEditorHandle>(null)

  const [page, setPage] = useState<Page>({ name: name ?? '', title: '', draft: false, content: '' })
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')
  const [saved, setSaved] = useState(false)
  const [editorKey, setEditorKey] = useState(0)

  useEffect(() => {
    if (!name) return
    api.getPage(name)
      .then(data => { setPage(data); setEditorKey(k => k + 1) })
      .catch(err => setError(err.message))
      .finally(() => setLoading(false))
  }, [name])

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSaved(false)

    const content = editorRef.current?.getMarkdown() ?? page.content
    const finalPage: Page = { ...page, content }

    try {
      await api.updatePage(name!, finalPage)
      setSaved(true)
      setTimeout(() => setSaved(false), 3000)
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : '保存失败')
    } finally {
      setSaving(false)
    }
  }

  if (loading) return <div className="page-loading">加载中...</div>

  const label = PAGE_LABELS[name ?? ''] ?? name

  return (
    <div className="edit-page">
      <div className="edit-header">
        <button className="btn-secondary" onClick={() => navigate('/pages')}>← 返回列表</button>
        <h2 className="page-title">编辑页面 · {label}</h2>
      </div>

      {error && <div className="alert alert-error">{error}</div>}
      {saved && <div className="alert alert-success">已保存</div>}

      <form onSubmit={handleSubmit} className="edit-form">
        <div className="edit-columns">
          {/* Left: meta */}
          <div className="meta-panel card">
            <h3 className="panel-title">页面信息</h3>

            <div className="form-group">
              <label htmlFor="title">标题 *</label>
              <input
                id="title"
                value={page.title}
                onChange={e => setPage(p => ({ ...p, title: e.target.value }))}
                placeholder="页面标题"
                required
              />
            </div>

            <div className="form-group draft-toggle">
              <label>
                <input
                  type="checkbox"
                  checked={page.draft}
                  onChange={e => setPage(p => ({ ...p, draft: e.target.checked }))}
                />
                <span>保存为草稿</span>
              </label>
            </div>

            <button type="submit" className="btn-primary save-btn" disabled={saving}>
              {saving ? '保存中...' : '保存更改'}
            </button>
          </div>

          {/* Right: editor */}
          <div className="editor-panel">
            <div className="editor-label">正文（Markdown）</div>
            <MilkdownEditor
              key={editorKey}
              ref={editorRef}
              defaultValue={page.content}
              onChange={md => setPage(p => ({ ...p, content: md }))}
            />
          </div>
        </div>
      </form>
    </div>
  )
}
