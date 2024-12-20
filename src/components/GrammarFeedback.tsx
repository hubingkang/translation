import React from 'react';

interface GrammarFeedbackProps {
  feedback: string | null;
  isLoading: boolean;
}

export const GrammarFeedback: React.FC<GrammarFeedbackProps> = ({ feedback, isLoading }) => {
  if (isLoading) {
    return (
      <div className="fixed bottom-4 right-4 bg-white p-4 rounded-lg shadow-lg max-w-md">
        <div className="animate-pulse">Checking grammar...</div>
      </div>
    );
  }

  if (!feedback) {
    return null;
  }

  return (
    <div className="fixed bottom-4 right-4 bg-white p-4 rounded-lg shadow-lg max-w-md">
      <h3 className="text-lg font-semibold mb-2">Grammar Check Results</h3>
      <p className="text-gray-700">{feedback}</p>
    </div>
  );
};
