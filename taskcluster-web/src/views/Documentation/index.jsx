import { hot } from 'react-hot-loader';
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import { lowerCase } from 'change-case';
import resolve from 'resolve-pathname';
import { withStyles } from '@material-ui/core/styles';
import { join } from 'path';
import 'prismjs/themes/prism.css';
import Dashboard from '../../components/Dashboard';
import ErrorPanel from '../../components/ErrorPanel';
import { DOCS_PATH_PREFIX, DOCS_MENU_ITEMS } from '../../utils/constants';
import docsTableOfContents from '../../autogenerated/docsTableOfContents';
import PageMeta from './PageMeta';
import NotFound from '../../components/NotFound';

@hot(module)
@withStyles(theme => ({
  warningPanel: {
    ...theme.mixins.warningPanel,
  },
}))
export default class Documentation extends Component {
  state = {
    error: null,
    Document: null,
    pageInfo: null,
  };

  componentDidMount() {
    this.load();
  }

  componentDidUpdate(prevProps) {
    if (this.props.match.params.path === prevProps.match.params.path) {
      return;
    }

    this.load();
  }

  anchorFactory = ({ href, ...props }, ...children) => {
    if (href.startsWith('http')) {
      return (
        <a href={href} {...props} target="_blank" rel="noopener noreferrer">
          {children}
        </a>
      );
    }

    const { location } = this.props;
    const url = resolve(href, location.pathname);

    return (
      <Link to={url} {...props}>
        {children}
      </Link>
    );
  };

  /* eslint-disable react/no-danger */
  codeBlockFactory = (props, children) => (
    <pre {...props}>
      <code dangerouslySetInnerHTML={{ __html: children }} />
    </pre>
  );
  /* eslint-enable react/no-danger */

  handlePageChange = url =>
    this.props.history.push(join(DOCS_PATH_PREFIX, url));

  findChildFromRootNode(node) {
    const currentPath = window.location.pathname.replace(
      `${DOCS_PATH_PREFIX}/`,
      ''
    );

    if (currentPath === node.path) {
      return node;
    }

    if (node.children) {
      for (let i = 0; i < node.children.length; i += 1) {
        const result = this.findChildFromRootNode(node.children[i]);

        if (result) {
          return result;
        }
      }
    }
  }

  getPageInfo() {
    const menuItem = DOCS_MENU_ITEMS.find(
      ({ path }) =>
        window.location.pathname !== DOCS_PATH_PREFIX &&
        path !== DOCS_PATH_PREFIX &&
        window.location.pathname.startsWith(path)
    );

    if (!menuItem) {
      return null;
    }

    const rootNode = docsTableOfContents[lowerCase(menuItem.label)];

    return this.findChildFromRootNode(rootNode) || rootNode;
  }

  async load() {
    try {
      const { default: Document } = await this.handleImport(
        this.props.match.params.path
      );
      const pageInfo = this.getPageInfo();

      this.setState({ Document, pageInfo, error: null });
    } catch (error) {
      this.setState({ error });
    }
  }

  async handleImport(url) {
    const doc = url ? url.replace(/\/$/, '') : 'index';

    return import(/* webpackChunkName: 'Documentation.page' */ `../../docs/${doc}.md`).catch(
      () =>
        import(/* webpackChunkName: 'Documentation.page' */ `../../docs/${doc}/index.md`)
    );
  }

  render() {
    const { classes, history } = this.props;
    const { error, Document, pageInfo } = this.state;

    return (
      <Dashboard
        docs
        title={
          pageInfo && pageInfo.data.title
            ? pageInfo.data.title
            : 'Documentation'
        }>
        <ErrorPanel
          warning
          error="Please refer to [https://docs.taskcluster.net/docs](https://docs.taskcluster.net/docs)
        for the documentation. The following is work in progress."
          className={classes.warningPanel}
        />
        {process.env.NODE_ENV !== 'production' && <ErrorPanel error={error} />}
        {error && <NotFound isDocs />}
        {Document && (
          <Document
            factories={{
              a: this.anchorFactory,
              codeBlock: this.codeBlockFactory,
            }}
          />
        )}
        {pageInfo && (
          <PageMeta
            pageInfo={pageInfo}
            history={history}
            onPageChange={this.handlePageChange}
          />
        )}
      </Dashboard>
    );
  }
}
