import { useNavigate } from 'react-router-dom'
import './PagesPage.css'

interface PageDef {
  name: string
  label: string
  description: string
}

const PAGES: PageDef[] = [
  { name: 'about', label: '关于', description: '关于页面（/about）' },
  { name: 'archives', label: '归档', description: '归档页面（/archives）' },
]

export default function PagesPage() {
  const navigate = useNavigate()

  return (
    <div className="pages-page">
      <div className="page-header">
        <h2 className="page-title">页面管理</h2>
      </div>

      <div className="pages-list card">
        {PAGES.map(p => (
          <div key={p.name} className="page-item">
            <div className="page-item-info">
              <span className="page-item-label">{p.label}</span>
              <span className="page-item-desc">{p.description}</span>
            </div>
            <button
              className="btn-secondary btn-sm"
              onClick={() => navigate(`/pages/${p.name}`)}
            >
              编辑
            </button>
          </div>
        ))}
      </div>
    </div>
  )
}
