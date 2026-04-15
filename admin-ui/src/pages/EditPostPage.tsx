import { useState, useEffect, useRef, FormEvent } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { api, Post } from '../api'
import MilkdownEditor, { MilkdownEditorHandle } from '../components/MilkdownEditor'
import './EditPostPage.css'

function todayIso() {
  return new Date().toISOString().slice(0, 19) + 'Z'
}

function slugify(title: string) {
  return title
    .toLowerCase()
    .replace(/[\u4e00-\u9fa5]+/g, match => encodeURIComponent(match).replace(/%/g, '').slice(0, 20))
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    || 'post-' + Date.now()
}

const emptyPost = (): Post => ({
  meta: {
    slug: '',
    title: '',
    date: todayIso(),
    description: '',
    categories: [],
    tags: [],
    draft: false,
  },
  content: '',
})

export default function EditPostPage() {
  const { slug } = useParams<{ slug: string }>()
  const navigate = useNavigate()
  const isNew = !slug
  const editorRef = useRef<MilkdownEditorHandle>(null)

  const [post, setPost] = useState<Post>(emptyPost())
  const [loading, setLoading] = useState(!isNew)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')
  const [saved, setSaved] = useState(false)
  const [editorKey, setEditorKey] = useState(0)

  useEffect(() => {
    if (!slug) return
    api.getPost(slug)
      .then(data => { setPost(data); setEditorKey(k => k + 1) })
      .catch(err => setError(err.message))
      .finally(() => setLoading(false))
  }, [slug])

  const updateMeta = <K extends keyof Post['meta']>(key: K, val: Post['meta'][K]) => {
    setPost(p => ({ ...p, meta: { ...p.meta, [key]: val } }))
  }

  const handleTitleChange = (title: string) => {
    updateMeta('title', title)
    if (isNew && !post.meta.slug) {
      updateMeta('slug', slugify(title))
    }
  }

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSaved(false)

    const content = editorRef.current?.getMarkdown() ?? post.content
    const finalPost: Post = { ...post, content }

    try {
      if (isNew) {
        const res = await api.createPost(finalPost)
        navigate(`/posts/${res.slug}`, { replace: true })
      } else {
        await api.updatePost(slug!, finalPost)
        setSaved(true)
        setTimeout(() => setSaved(false), 3000)
      }
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : '保存失败')
    } finally {
      setSaving(false)
    }
  }

  if (loading) return <div className="page-loading">加载中...</div>

  return (
    <div className="edit-page">
      <div className="edit-header">
        <button className="btn-secondary" onClick={() => navigate('/posts')}>← 返回列表</button>
        <h2 className="page-title">{isNew ? '新建文章' : '编辑文章'}</h2>
      </div>

      {error && <div className="alert alert-error">{error}</div>}
      {saved && <div className="alert alert-success">已保存</div>}

      <form onSubmit={handleSubmit} className="edit-form">
        <div className="edit-columns">
          {/* Left: meta */}
          <div className="meta-panel card">
            <h3 className="panel-title">文章信息</h3>

            <div className="form-group">
              <label htmlFor="title">标题 *</label>
              <input
                id="title"
                value={post.meta.title}
                onChange={e => handleTitleChange(e.target.value)}
                placeholder="文章标题"
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="slug">Slug (URL)</label>
              <input
                id="slug"
                value={post.meta.slug}
                onChange={e => updateMeta('slug', e.target.value.replace(/[^a-zA-Z0-9_\-]/g, ''))}
                placeholder="article-slug"
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="date">日期</label>
              <input
                id="date"
                type="datetime-local"
                value={post.meta.date.slice(0, 16)}
                onChange={e => updateMeta('date', e.target.value + ':00Z')}
              />
            </div>

            <div className="form-group">
              <label htmlFor="desc">摘要</label>
              <textarea
                id="desc"
                rows={3}
                value={post.meta.description}
                onChange={e => updateMeta('description', e.target.value)}
                placeholder="文章简短描述（可选）"
              />
            </div>

            <div className="form-group">
              <label htmlFor="categories">分类（逗号分隔）</label>
              <input
                id="categories"
                value={post.meta.categories.join(', ')}
                onChange={e => updateMeta('categories', e.target.value.split(',').map(s => s.trim()).filter(Boolean))}
                placeholder="技术, 随笔"
              />
            </div>

            <div className="form-group">
              <label htmlFor="tags">标签（逗号分隔）</label>
              <input
                id="tags"
                value={post.meta.tags.join(', ')}
                onChange={e => updateMeta('tags', e.target.value.split(',').map(s => s.trim()).filter(Boolean))}
                placeholder="rust, hugo"
              />
            </div>

            <div className="form-group draft-toggle">
              <label>
                <input
                  type="checkbox"
                  checked={post.meta.draft}
                  onChange={e => updateMeta('draft', e.target.checked)}
                />
                <span>保存为草稿</span>
              </label>
            </div>

            <button type="submit" className="btn-primary save-btn" disabled={saving}>
              {saving ? '保存中...' : (isNew ? '创建文章' : '保存更改')}
            </button>
          </div>

          {/* Right: editor */}
          <div className="editor-panel">
            <div className="editor-label">正文（Markdown）</div>
            <MilkdownEditor
              key={editorKey}
              ref={editorRef}
              defaultValue={post.content}
              onChange={md => setPost(p => ({ ...p, content: md }))}
            />
          </div>
        </div>
      </form>
    </div>
  )
}
