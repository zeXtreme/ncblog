import { useState, useEffect, FormEvent } from 'react'
import { api, SiteConfig } from '../api'
import './SiteConfigPage.css'

const defaults: SiteConfig = {
  title: '',
  base_url: '',
  description: '',
  author: '',
  bio: '',
  github: '',
  twitter: '',
  email: '',
}

export default function SiteConfigPage() {
  const [cfg, setCfg] = useState<SiteConfig>(defaults)
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')
  const [saved, setSaved] = useState(false)

  useEffect(() => {
    api.getSiteConfig()
      .then(data => setCfg(data))
      .catch(err => setError(err.message))
      .finally(() => setLoading(false))
  }, [])

  const update = (key: keyof SiteConfig, val: string) => {
    setCfg(c => ({ ...c, [key]: val }))
  }

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSaved(false)
    try {
      await api.updateSiteConfig(cfg)
      setSaved(true)
      setTimeout(() => setSaved(false), 3000)
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : '保存失败')
    } finally {
      setSaving(false)
    }
  }

  if (loading) return <div className="page-loading">加载中...</div>

  return (
    <div className="site-config-page">
      <h2 className="page-title" style={{ marginBottom: '1.5rem' }}>站点设置</h2>

      {error && <div className="alert alert-error">{error}</div>}
      {saved && <div className="alert alert-success">已保存</div>}

      <form onSubmit={handleSubmit} className="config-form card">
        <section className="config-section">
          <h3 className="section-title">基本信息</h3>
          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="title">站点标题</label>
              <input id="title" value={cfg.title} onChange={e => update('title', e.target.value)} placeholder="My Blog" />
            </div>
            <div className="form-group">
              <label htmlFor="base_url">站点 URL</label>
              <input id="base_url" value={cfg.base_url} onChange={e => update('base_url', e.target.value)} placeholder="https://example.com/" />
            </div>
            <div className="form-group full">
              <label htmlFor="description">站点描述</label>
              <textarea id="description" rows={2} value={cfg.description} onChange={e => update('description', e.target.value)} placeholder="站点简介" />
            </div>
          </div>
        </section>

        <section className="config-section">
          <h3 className="section-title">作者信息</h3>
          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="author">作者名称</label>
              <input id="author" value={cfg.author} onChange={e => update('author', e.target.value)} placeholder="Your Name" />
            </div>
            <div className="form-group">
              <label htmlFor="email">邮箱</label>
              <input id="email" type="email" value={cfg.email} onChange={e => update('email', e.target.value)} placeholder="you@example.com" />
            </div>
            <div className="form-group full">
              <label htmlFor="bio">个人简介</label>
              <textarea id="bio" rows={2} value={cfg.bio} onChange={e => update('bio', e.target.value)} placeholder="一两句自我介绍" />
            </div>
          </div>
        </section>

        <section className="config-section">
          <h3 className="section-title">社交链接</h3>
          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="github">GitHub 用户名</label>
              <input id="github" value={cfg.github} onChange={e => update('github', e.target.value)} placeholder="username" />
            </div>
            <div className="form-group">
              <label htmlFor="twitter">Twitter / X 用户名</label>
              <input id="twitter" value={cfg.twitter} onChange={e => update('twitter', e.target.value)} placeholder="username" />
            </div>
          </div>
        </section>

        <div className="form-footer">
          <button type="submit" className="btn-primary" disabled={saving}>
            {saving ? '保存中...' : '保存设置'}
          </button>
        </div>
      </form>
    </div>
  )
}
