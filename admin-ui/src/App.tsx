import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import LoginPage from './pages/LoginPage'
import PostsPage from './pages/PostsPage'
import EditPostPage from './pages/EditPostPage'
import PagesPage from './pages/PagesPage'
import EditPagePage from './pages/EditPagePage'
import SiteConfigPage from './pages/SiteConfigPage'
import Layout from './components/Layout'

export default function App() {
  return (
    <BrowserRouter basename="/admin">
      <Routes>
        <Route path="/login" element={<LoginPage />} />
        <Route element={<Layout />}>
          <Route path="/" element={<Navigate to="/posts" replace />} />
          <Route path="/posts" element={<PostsPage />} />
          <Route path="/posts/new" element={<EditPostPage />} />
          <Route path="/posts/:slug" element={<EditPostPage />} />
          <Route path="/pages" element={<PagesPage />} />
          <Route path="/pages/:name" element={<EditPagePage />} />
          <Route path="/site-config" element={<SiteConfigPage />} />
        </Route>
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  )
}
