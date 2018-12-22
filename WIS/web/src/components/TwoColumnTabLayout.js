import React, { Component } from 'react';

export default class TwoColumnTabLayout extends Component {
  render() {
    return (
      <div className="two-column-tab-content">
        {this.props.children}
      </div>
    );
  }
}
