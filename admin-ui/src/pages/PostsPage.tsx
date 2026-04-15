import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { api, PostMeta } from '../api'
import './PostsPage.css'

export default function PostsPage() {
  const navigate = useNavigate()
  const [posts, setPosts] = useState<PostMeta[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [buildMsg, setBuildMsg] = useState('')
  const [building, setBuilding] = useState(false)

  const loadPosts = async () => {
    try {
      const data = await api.listPosts()
      setPosts(data)
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : '加载失败')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => { loadPosts() }, [])

  const handleDelete = async (slug: string, title: string) => {
    if (!confirm(`确认删除文章「${title}」？`)) return
    try {
      await api.deletePost(slug)
      setPosts(posts => posts.filter(p => p.slug !== slug))
    } catch (err: unknown) {
      alert(err instanceof Error ? err.message : '删除失败')
    }
  }

  const handleBuild = async () => {
    setBuilding(true)
    setBuildMsg('')
    try {
      const res = await api.triggerBuild()
      if (res.success) {
        setBuildMsg('构建成功！')
      } else {
        setBuildMsg(`构建失败: ${res.stderr || res.error || '未知错误'}`)
      }
    } catch (err: unknown) {
      setBuildMsg(err instanceof Error ? err.message : '构建请求失败')
    } finally {
      setBuilding(false)
    }
  }

  const formatDate = (iso: string) => {
    try { return new Date(iso).toLocaleDateString('zh-CN') } catch { return iso }
  }

  if (loading) return <div className="page-loading">加载中...</div>

  return (
    <div className="posts-page">
      <div className="page-header">
        <h2 className="page-title">文章管理</h2>
        <div className="header-actions">
          {buildMsg && (
            <span className={`build-msg ${buildMsg.includes('成功') ? 'success' : 'error'}`}>
              {buildMsg}
            </span>
          )}
          <button className="btn-secondary" onClick={handleBuild} disabled={building}>
            {building ? '构建中...' : '触发构建'}
          </button>
          <button className="btn-primary" onClick={() => navigate('/posts/new')}>
            新建文章
          </button>
        </div>
      </div>

      {error && <div className="alert alert-error">{error}</div>}

      {posts.length === 0 ? (
        <div className="empty-state card">
          <p>还没有文章，点击「新建文章」开始写作。</p>
        </div>
      ) : (
        <div className="posts-table card">
          <table>
            <thead>
              <tr>
                <th>标题</th>
                <th>日期</th>
                <th>分类</th>
                <th>状态</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {posts.map(post => (
                <tr key={post.slug}>
                  <td className="post-title-cell">
                    <a href="#" onClick={e => { e.preventDefault(); navigate(`/posts/${post.slug}`) }}>
                      {post.title || post.slug}
                    </a>
                  </td>
                  <td className="post-date">{formatDate(post.date)}</td>
                  <td className="post-cats">{post.categories.join(', ') || '—'}</td>
                  <td>
                    <span className={`status-badge ${post.draft ? 'draft' : 'published'}`}>
                      {post.draft ? '草稿' : '已发布'}
                    </span>
                  </td>
                  <td className="post-actions">
                    <button className="btn-secondary btn-sm" onClick={() => navigate(`/posts/${post.slug}`)}>
                      编辑
                    </button>
                    <button className="btn-danger btn-sm" onClick={() => handleDelete(post.slug, post.title)}>
                      删除
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
