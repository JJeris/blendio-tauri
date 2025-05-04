import React from 'react';
import { BrowserRouter as Router, useLocation } from 'react-router-dom';
import AppRouter from './router';
import TitleBar from './components/titleBar/TitleBar';

const AppContent = () => {
    const location = useLocation();
    // Only show title bar if we're not in a popup route
    const showTitleBar = !location.pathname.startsWith('/popup');
    return (
        <>
            {showTitleBar && <TitleBar />}
            <AppRouter />
        </>
    );
};
const App = () => (
    <Router>
        <AppContent />
    </Router>
);
export default App;
