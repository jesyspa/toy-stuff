#pragma once

#include <functional>
#include <memory>
#include <vector>

// This is the key class here.  A GeoNode represents an area of screen space,
// which may be associated with visualisation and behaviour.
struct GeoNode;

struct Vec2i {
  int x, y;
};

Vec2i operator+(Vec2i a, Vec2i b) { return {a.x + b.x, a.y + b.y}; }
Vec2i operator-(Vec2i a, Vec2i b) { return {a.x - b.x, a.y - b.y}; }
bool operator<=(Vec2i a, Vec2i b) { return a.x <= b.x && a.y <= b.y; }

struct DrawContext {
  DrawContext(Vec2i displacement, Vec2i bounds)
      : displacement_{displacement}, bounds_{bounds} {}

  Vec2i resolve(Vec2i pos) const { return displacement_ + pos; }
  Vec2i get_bounds() const { return bounds_; }

 private:
  Vec2i displacement_;
  Vec2i bounds_;
};

using DrawAction = std::function<void(const DrawContext&)>;
using ClickAction = std::function<void(Vec2i)>;

class GeoNode {
 public:
  GeoNode* AddNode(Vec2i position, Vec2i size) {
    children_.emplace_back(std::make_unique<GeoNode>(position, size));
    return children_.back().get();
  }

  void SetDraw(DrawAction) { /* ... */ }
  void SetClick(ClickAction) { /* ... */ }

  // needs to be public for make_unique, bit unfortunate.
  GeoNode(Vec2i position, Vec2i size) : position_{position}, size_{size} {}

 private:
  friend class GeoRoot;
  void Draw(Vec2i pos) const {
    if (draw_action_) {
      DrawContext context(pos + position_, size_);
      draw_action_(context);
    }
    for (const auto& child : children_) child->Draw(pos + position_);
  }

  void Click(Vec2i pos) const {
    GeoNode* child = FindChildAt(pos);
    if (child) {
      child->Click(pos - position_);
    } else if (click_action_) {
      click_action_(pos);
    }
  }

  GeoNode* FindChildAt(Vec2i pos) const {
    // TODO: use <algorithm>
    for (const auto& child : children_)
      if (child->position_ <= pos && pos <= child->position_ + child->size_)
        return child.get();
    return nullptr;
  }

  Vec2i position_, size_;
  std::vector<std::unique_ptr<GeoNode>> children_;
  DrawAction draw_action_;
  ClickAction click_action_;
};

class GeoRoot {
 public:
  GeoRoot(Vec2i size) : node_{{}, size} {}

  void Draw() const { node_.Draw({0, 0}); }
  void Click(Vec2i pos) const { node_.Click(pos); }
  GeoNode* AddNode(Vec2i position, Vec2i size) {
    return node_.AddNode(position, size);
  }

 private:
  GeoNode node_;
};
