import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable from 'material-react-table';

import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Routes, Route, Outlet, Link, createRoutesFromElements, createBrowserRouter, RouterProvider, Router } from "react-router-dom";
import { Sidebar, Menu, MenuItem } from 'react-pro-sidebar';

import MenuOpenIcon from '@mui/icons-material/MenuOpen';
import MenuIcon from '@mui/icons-material/Menu';
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import InboxIcon from '@mui/icons-material/MoveToInbox';
import MailIcon from '@mui/icons-material/Mail';
import MuiDrawer from '@mui/material/Drawer';
import SettingsSuggestIcon from '@mui/icons-material/SettingsSuggest';
import SettingsRemoteIcon from '@mui/icons-material/SettingsRemote';


import { styled, useTheme, Theme, CSSObject } from '@mui/material/styles';
import MuiAppBar, { AppBarProps as MuiAppBarProps } from '@mui/material/AppBar';

import { useNavigate } from 'react-router-dom';


import "./App.css";
import LocalExplorer from "./local/local_explorer";
import ErrorBoundary from "./errors/default_boundary";
import { Box, CssBaseline, Divider, IconButton, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Toolbar, Typography } from "@mui/material";
import Layout from "./Layout";
import S3Enpoints from "./s3/s3_endpoints";
import S3FilesExplorer from "./s3/s3_files_explorer";
import S3Explorer from "./s3/s3_explorer";



function App() {


  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Home />} />
        <Route path="local" element={<LocalExplorer />} />
        <Route path="s3-endpoints" element={<S3Enpoints/>} />
        <Route path="s3-files-explorer" element={<S3FilesExplorer/>} />
        <Route path="s3-explorer" element={<S3Explorer/>} />
        <Route path="settings" element={<Settings />} />
        <Route path="*" element={<NoMatch />} />
      </Route>
    </Routes>
  );


}



function Home() {
  return (
    <div>
      <h2>Home</h2>
    </div>
  );
}

function Settings() {
  return (
    <div>
      <h2>Not yet implemented</h2>
    </div>
  );
}

function NoMatch() {
  return (
    <div>
      <h2>Nothing to see here!</h2>
      <p>
        <Link to="/">Go to the home page</Link>
      </p>
    </div>
  );
}

export default App;
