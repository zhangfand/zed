import React from 'react';

type Props = {
  newComment: string;
  handleAddComment: () => void;
  setNewComment: (value: string) => void;
};

const Composer: React.FC<Props> = ({ newComment, handleAddComment, setNewComment }) => {
  return (
    <div className="mt-4">
      <textarea
        value={newComment}
        onChange={(e) => setNewComment(e.target.value)}
        className="w-full h-20 bg-white border rounded-md shadow-md p-2"
        placeholder="Write a comment..."
      ></textarea>
      <button
        onClick={handleAddComment}
        className="bg-blue-500 text-white px-4 py-2 rounded-md mt-2"
      >
        Add Comment
      </button>
    </div>
  );
};

export default Composer;
