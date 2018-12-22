import React, { Component } from 'react';

export default class TabTwoColumnLayout extends Component {
  render() {
    return (
      <div className="tab-content">
        <div className="tab-content-fixed">{this.props.children[0]}</div>
        <div className="tab-content-scrollable">{this.props.children[1]}</div>
      </div>
    );
  }
}
