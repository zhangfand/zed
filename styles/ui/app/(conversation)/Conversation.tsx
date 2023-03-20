import React, { useState } from 'react';
import Comment from './Comment';
import { useConversationStore } from '../../stores/conversationStore';
import Composer from './Composer';

const Conversation: React.FC = () => {
    const comments = useConversationStore((state) => state.comments);
    const addComment = useConversationStore((state) => state.addComment);
    const deleteComment = useConversationStore((state) => state.deleteComment);
    const [newComment, setNewComment] = useState('');

    const handleAddComment = () => {
        if (newComment.trim()) {
            addComment(newComment);
            setNewComment('');
        }
    };

    return (
        <div className="space-y-4">
            <div className="overflow-y-auto max-h-[600px] my-px">
                {comments.map((comment) => (
                    <Comment key={comment.id} comment={comment} onDelete={deleteComment} />
                ))}
            </div>
            <Composer
                newComment={newComment}
                handleAddComment={handleAddComment}
                setNewComment={setNewComment}
            />
        </div>
    );
};

export default Conversation;
