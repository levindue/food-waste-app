import React from 'react';

// dropdown context for open state
const DropdownContext = React.createContext({
    open: false,
    setOpen: () => {},
});

// dropdown component for wrapping and providing context
function Dropdown({ children, ...props }) {
  const [open, setOpen] = React.useState(false);
  return (
     <DropdownContext.Provider value={{ open, setOpen }}>
       <div className="relative">{children}</div>
     </DropdownContext.Provider>
  );
};