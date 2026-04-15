export interface PostMeta {
  slug: string
  title: string
  date: string
  description: string
  categories: string[]
  tags: string[]
  draft: boolean
}

export interface Post {
  meta: PostMeta
  content: string
}

export interface Page {
  name: string
  title: string
  draft: boolean
  content: string
}

export interface SiteConfig {
  title: string
  base_url: string
  description: string
  author: string
  bio: string
  github: string
  twitter: string
  email: string
}

export interface BuildResult {
  success: boolean
  stdout?: string
  stderr?: string
  error?: string
}

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(path, {
    credentials: 'include',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    ...options,
  })

  if (res.status === 401) {
    window.location.href = '/admin/login'
    throw new Error('未授权')
  }

  const data = await res.json()
  if (!res.ok) {
    throw new Error(data.error || `HTTP ${res.status}`)
  }
  return data
}

export const api = {
  // Auth
  login: (password: string) =>
    request<{ success: boolean; message: string }>('/api/login', {
      method: 'POST',
      body: JSON.stringify({ password }),
    }),

  logout: () => request('/api/logout', { method: 'POST' }),

  me: () => request<{ authenticated: boolean }>('/api/me'),

  // Posts
  listPosts: () => request<PostMeta[]>('/api/posts'),

  getPost: (slug: string) => request<Post>(`/api/posts/${slug}`),

  createPost: (post: Post) =>
    request<{ success: boolean; slug: string }>('/api/posts', {
      method: 'POST',
      body: JSON.stringify(post),
    }),

  updatePost: (slug: string, post: Post) =>
    request<{ success: boolean }>(`/api/posts/${slug}`, {
      method: 'PUT',
      body: JSON.stringify(post),
    }),

  deletePost: (slug: string) =>
    request<{ success: boolean }>(`/api/posts/${slug}`, {
      method: 'DELETE',
    }),

  // Pages
  getPage: (name: string) => request<Page>(`/api/pages/${name}`),

  updatePage: (name: string, page: Page) =>
    request<{ success: boolean }>(`/api/pages/${name}`, {
      method: 'PUT',
      body: JSON.stringify(page),
    }),

  // Site config
  getSiteConfig: () => request<SiteConfig>('/api/site-config'),

  updateSiteConfig: (cfg: SiteConfig) =>
    request<{ success: boolean }>('/api/site-config', {
      method: 'PUT',
      body: JSON.stringify(cfg),
    }),

  // Build
  triggerBuild: () => request<BuildResult>('/api/build', { method: 'POST' }),
}
