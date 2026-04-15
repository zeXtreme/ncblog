import { useEffect, useState } from 'react'
import { Outlet, NavLink, useNavigate } from 'react-router-dom'
import { api } from '../api'
import './Layout.css'

export default function Layout() {
  const navigate = useNavigate()
  const [checking, setChecking] = useState(true)

  useEffect(() => {
    api.me().catch(() => navigate('/admin/login')).finally(() => setChecking(false))
  }, [navigate])

  const handleLogout = async () => {
    await api.logout().catch(() => {})
    navigate('/admin/login')
  }

  if (checking) return <div className="loading">加载中...</div>

  return (
    <div className="admin-layout">
      <nav className="admin-nav">
        <div className="nav-brand">ncblog</div>
        <div className="nav-links">
          <NavLink to="/posts" className={({ isActive }) => isActive ? 'nav-link active' : 'nav-link'}>
            文章
          </NavLink>
          <NavLink to="/pages" className={({ isActive }) => isActive ? 'nav-link active' : 'nav-link'}>
            页面
          </NavLink>
          <NavLink to="/site-config" className={({ isActive }) => isActive ? 'nav-link active' : 'nav-link'}>
            站点设置
          </NavLink>
        </div>
        <button className="btn-secondary nav-logout" onClick={handleLogout}>退出</button>
      </nav>
      <main className="admin-main">
        <Outlet />
      </main>
    </div>
  )
}
