'use client'

import {
  useDislikePost,
  useGetPosts,
  useLikePost,
  useRemovePost,
  useRemovePostReaction,
} from '@/components/post/post-data-access'
import { CommentsSection } from '@/components/post/post-ui'
import { Button } from '@/components/ui/button'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import { useState } from 'react'

export default function PostPage() {
  const params = useParams()
  const id = params.id as string
  const { data: posts, isLoading, isError } = useGetPosts()

  const removePost = useRemovePost()
  const likePost = useLikePost()
  const dislikePost = useDislikePost()
  const removeReaction = useRemovePostReaction()
  const [removingTopic, setRemovingTopic] = useState<string | null>(null)
  const [reactingPubkey, setReactingPubkey] = useState<string | null>(null)

  if (isLoading) {
    return (
      <div className="container mx-auto py-8">
        <p>Loading post...</p>
      </div>
    )
  }

  if (isError || !posts) {
    return (
      <div className="container mx-auto py-8">
        <p>Error loading post.</p>
      </div>
    )
  }

  const post = posts.find((p) => p.pubkey.toBase58() === id)

  if (!post) {
    return (
      <div className="container mx-auto py-8">
        <p>Post not found.</p>
        <Link href="/">
          <Button>Back to Posts</Button>
        </Link>
      </div>
    )
  }

  const handleRemove = async (topic: string) => {
    try {
      setRemovingTopic(topic)
      await removePost.mutateAsync({ topic })
    } catch (error) {
      console.error('Failed to remove post:', error)
    } finally {
      setRemovingTopic(null)
    }
  }

  const handleLike = async (postPubkey: string) => {
    try {
      setReactingPubkey(postPubkey)
      await likePost.mutateAsync({ postPubkey: post.pubkey })
    } catch (error) {
      console.error('Failed to like post:', error)
    } finally {
      setReactingPubkey(null)
    }
  }

  const handleDislike = async (postPubkey: string) => {
    try {
      setReactingPubkey(postPubkey)
      await dislikePost.mutateAsync({ postPubkey: post.pubkey })
    } catch (error) {
      console.error('Failed to dislike post:', error)
    } finally {
      setReactingPubkey(null)
    }
  }

  const handleRemoveReaction = async (postPubkey: string) => {
    try {
      setReactingPubkey(postPubkey)
      await removeReaction.mutateAsync({ postPubkey: post.pubkey })
    } catch (error) {
      console.error('Failed to remove reaction:', error)
    } finally {
      setReactingPubkey(null)
    }
  }

  const postKey = post.pubkey.toBase58()
  const isReacting = reactingPubkey === postKey

  return (
    <div className="container mx-auto py-8 space-y-8">
      <Link href="/">
        <Button variant="outline">← Back to Posts</Button>
      </Link>

      <div className="max-w-2xl mx-auto p-6 bg-card rounded-lg border shadow-sm">
        <div className="flex justify-between items-start mb-2">
          <h1 className="text-2xl font-bold">{post.topic}</h1>
          <Button
            variant="destructive"
            size="sm"
            onClick={() => handleRemove(post.topic)}
            disabled={removingTopic === post.topic}
          >
            {removingTopic === post.topic ? 'Removing...' : 'Remove'}
          </Button>
        </div>
        <p className="text-sm text-muted-foreground mb-2">By: {post.author.toBase58()}</p>
        <p className="text-base mb-4">{post.content}</p>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={() => handleLike(postKey)} disabled={isReacting}>
            {isReacting && likePost.isPending ? 'Liking...' : `Like (${post.likes})`}
          </Button>
          <Button variant="outline" size="sm" onClick={() => handleDislike(postKey)} disabled={isReacting}>
            {isReacting && dislikePost.isPending ? 'Disliking...' : `Dislike (${post.dislikes})`}
          </Button>
          <Button variant="outline" size="sm" onClick={() => handleRemoveReaction(postKey)} disabled={isReacting}>
            {isReacting && removeReaction.isPending ? 'Clearing...' : 'Clear Reaction'}
          </Button>
        </div>
      </div>

      <div className="max-w-2xl mx-auto">
        <CommentsSection postPubkey={post.pubkey} />
      </div>
    </div>
  )
}
