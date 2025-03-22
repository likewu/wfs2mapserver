//import { getPosts } from '@/lib/posts'
//import { Post } from '@/ui/post'
 
'use server'
 
export async function createPost(formData: FormData) {
  const title = formData.get('title')
  const content = formData.get('content')
 
  // Update data
  // Revalidate cache
}