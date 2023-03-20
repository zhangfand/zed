// stores/conversationStore.ts

import create from 'zustand';
import { Comment, sampleComments } from '../data/conversations';

type ConversationState = {
    comments: Comment[];
    addComment: (message: string, author?: string) => void;
    deleteComment: (id: number) => void;
};

export const useConversationStore = create<ConversationState>((set) => ({
    comments: sampleComments,
    addComment: (message, author = 'User') => {
        set((state) => {
            const newId = Math.max(...state.comments.map((comment) => comment.id)) + 1;
            const newTimestamp = new Date().toISOString();
            return {
                comments: [...state.comments, { id: newId, author, message, timestamp: newTimestamp }],
            };
        });
    },
    deleteComment: (id) => {
        set((state) => {
            return {
                comments: state.comments.filter((comment) => comment.id !== id),
            };
        });
    },
}));
